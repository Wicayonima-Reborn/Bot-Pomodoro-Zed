use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use zed_extension_api as zed;

// Struktur datanya
#[derive(Debug, Clone)]
struct Session {
    start_timestamp: u64,
    end_timestamp: Option<u64>,
    duration_seconds: u64,
    date: String,
}

#[derive(Debug)]
struct TrackerData {
    sessions: Vec<Session>,
    total_time_seconds: u64,
}

impl TrackerData {
    fn new() -> Self {
        Self {
            sessions: Vec::new(),
            total_time_seconds: 0,
        }
    }

    fn add_session(&mut self, session: Session) {
        self.total_time_seconds += session.duration_seconds;
        self.sessions.push(session);
    }

    fn get_today_total(&self) -> u64 {
        let today = Self::get_current_date();
        self.sessions
            .iter()
            .filter(|s| s.date == today)
            .map(|s| s.duration_seconds)
            .sum()
    }

    fn get_current_date() -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Simple date calculation (UTC)
        let days_since_epoch = now / 86400;
        let years = days_since_epoch / 365;
        let remaining_days = days_since_epoch % 365;
        let month = (remaining_days / 30) + 1;
        let day = (remaining_days % 30) + 1;
        
        format!("{:04}-{:02}-{:02}", 1970 + years, month, day)
    }

    fn format_duration(seconds: u64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;
        
        if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, secs)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, secs)
        } else {
            format!("{}s", secs)
        }
    }
}

// Simpan
struct Storage {
    data_path: PathBuf,
}

impl Storage {
    fn new() -> Self {
        let data_dir = std::env::var("APPDATA")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".to_string());
        
        let mut data_path = PathBuf::from(data_dir);
        data_path.push("Zed");
        data_path.push("coding-tracker-data.txt");
        Self { data_path }
    }

    fn load(&self) -> TrackerData {
        match fs::read_to_string(&self.data_path) {
            Ok(content) => self.parse_data(&content),
            Err(_) => {
                eprintln!("[Tracker] No existing data found, starting fresh");
                TrackerData::new()
            }
        }
    }

    fn save(&self, data: &TrackerData) {
        let mut content = String::new();
        content.push_str(&format!("TOTAL_SECONDS={}\n", data.total_time_seconds));   
        for session in &data.sessions {
            let end = session.end_timestamp.unwrap_or(0);
            content.push_str(&format!(
                "SESSION|{}|{}|{}|{}\n",
                session.date, session.start_timestamp, end, session.duration_seconds
            ));
        }

        if let Some(parent) = self.data_path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        match fs::write(&self.data_path, content) {
            Ok(_) => eprintln!("[Tracker] Data saved successfully"),
            Err(e) => eprintln!("[Tracker] Failed to save data: {}", e),
        }
    }

    fn parse_data(&self, content: &str) -> TrackerData {
        let mut data = TrackerData::new();
        
        for line in content.lines() {
            if line.starts_with("TOTAL_SECONDS=") {
                if let Some(value) = line.strip_prefix("TOTAL_SECONDS=") {
                    data.total_time_seconds = value.parse().unwrap_or(0);
                }
            } else if line.starts_with("SESSION|") {
                if let Some(session) = self.parse_session(line) {
                    data.sessions.push(session);
                }
            }
        }
        
        eprintln!("[Tracker] Loaded {} previous sessions", data.sessions.len());
        data
    }

    fn parse_session(&self, line: &str) -> Option<Session> {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != 5 {
            return None;
        }

        Some(Session {
            date: parts[1].to_string(),
            start_timestamp: parts[2].parse().ok()?,
            end_timestamp: {
                let val: u64 = parts[3].parse().ok()?;
                if val == 0 { None } else { Some(val) }
            },
            duration_seconds: parts[4].parse().ok()?,
        })
    }
}

// Tracker state
struct TrackerState {
    start_time: Instant,
    start_timestamp: u64,
    data: TrackerData,
    storage: Storage,
}

impl TrackerState {
    fn new() -> Self {
        let now = Instant::now();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let storage = Storage::new();
        let data = storage.load();
        
        eprintln!("[Tracker] Session started at {}", TrackerData::get_current_date());
        eprintln!("[Tracker] All-time total: {}", TrackerData::format_duration(data.total_time_seconds));
        eprintln!("[Tracker] Today's total: {}", TrackerData::format_duration(data.get_today_total()));
        
        Self {
            start_time: now,
            start_timestamp: timestamp,
            data,
            storage,
        }
    }

    fn elapsed_seconds(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    fn finalize(&mut self) {
        let duration = self.elapsed_seconds();
        let end_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let session = Session {
            start_timestamp: self.start_timestamp,
            end_timestamp: Some(end_timestamp),
            duration_seconds: duration,
            date: TrackerData::get_current_date(),
        };

        self.data.add_session(session);
        self.storage.save(&self.data);
    }

    fn print_summary(&self) {
        let current_duration = self.elapsed_seconds();
        let today_total = self.data.get_today_total() + current_duration;
        let all_time_total = self.data.total_time_seconds + current_duration;

        eprintln!("\n╔══════════════════════════════════════════════╗");
        eprintln!("║          Coding Session Summary            ║");
        eprintln!("╠══════════════════════════════════════════════╣");
        eprintln!("║  This Session: {:<28}║", TrackerData::format_duration(current_duration));
        eprintln!("║  Today Total:  {:<28}║", TrackerData::format_duration(today_total));
        eprintln!("║  All-Time:     {:<28}║", TrackerData::format_duration(all_time_total));
        eprintln!("║  Total Sessions: {:<26}║", self.data.sessions.len() + 1);
        eprintln!("╚══════════════════════════════════════════════╝\n");
    }

    fn print_periodic_update(&self) {
        let current = self.elapsed_seconds();
        let today_total = self.data.get_today_total() + current;
        
        eprintln!(
            "[Tracker] ⏰ Current: {} | Today: {} | All-Time: {}",
            TrackerData::format_duration(current),
            TrackerData::format_duration(today_total),
            TrackerData::format_duration(self.data.total_time_seconds + current)
        );
    }
}

// Extension
struct CodingTrackerExtension {
    state: Arc<Mutex<TrackerState>>,
}

impl zed::Extension for CodingTrackerExtension {
    fn new() -> Self {
        let state = Arc::new(Mutex::new(TrackerState::new()));
        
        eprintln!("[Tracker] Extension initialized!");
        
        // Background thread untuk periodic updates
        let state_clone = Arc::clone(&state);
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(600)); // Updatenya tiap 10 menit
                if let Ok(state) = state_clone.lock() {
                    state.print_periodic_update();
                }
            }
        });
        Self { state }
    }
}

impl Drop for CodingTrackerExtension {
    fn drop(&mut self) {
        if let Ok(mut state) = self.state.lock() {
            state.print_summary();
            state.finalize();
            eprintln!("[Tracker] Session saved!");
        }
    }
}
zed::register_extension!(CodingTrackerExtension);
🎵 CLI Music Player (Rust)

A simple command-line music player built with Rust, using rodio for audio playback. Supports playlist management, shuffle, repeat, and liking/unliking songs.

🚀 Features

✅ Scan a directory for audio files (.mp3, .wav)
✅ Play, pause, resume, stop tracks
✅ Next/previous track navigation
✅ Shuffle and repeat modes
✅ Like/unlike songs and view liked songs
✅ Search and play songs by keyword

🛠️ Installation

Prerequisites
	•	Install Rust and Cargo

Clone the Repository

git clone git clone git clone https://github.com/Erness23/Rust-project.git
cd Rust-project

Install Dependencies

cargo build

🎧 Usage

Run the music player:

cargo run

Follow the on-screen commands to interact with the player.

📂 Directory Setup

Ensure your music files are stored in a specified folder. Update the scan_all_audio_files function with your music directory path.

🔧 Configuration
	•	Change the default music directory in scan_all_audio_files()
	•	Modify supported file formats as needed

🏗 Dependencies

This project relies on the following Rust crates:
	•	rodio – For audio playback
	•	tokio – For asynchronous execution
	•	walkdir – For discovering audio files in directories
	•	rand – For shuffle functionality

🛠️ Future Improvements
	•	Volume control
	•	Support for more audio formats (.flac, .ogg, .aac)
	•	Persistent playlist storage

📜 License

This project is open-source under the MIT License.
 
use rodio::{OutputStream, Sink, Decoder};
use std::{
    fs::File,
    io::{self, BufReader, Write, BufRead},
    path::Path,
    sync::{Arc, Mutex},
};
use rand::seq::SliceRandom;
use rand::Rng;
use walkdir::WalkDir;

#[tokio::main]
async fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));

    let mut playlist = vec![];
    let mut liked_songs = load_liked_songs();
    let mut current_index = 0;
    let mut shuffle_mode = false;
    let mut repeat_mode = false;

    loop {
        println!("\nMusic Player Commands:");
        println!("1. Scan all audio files");
        println!("2. Play current song");
        println!("3. Pause");
        println!("4. Resume");
        println!("5. Stop");
        println!("6. Next Track");
        println!("7. Previous Track");
        println!("8. Toggle Like/Unlike Current Song");
        println!("9. Show Liked Songs");
        println!("10. Toggle Shuffle Mode ({})", if shuffle_mode { "On" } else { "Off" });
        println!("11. Toggle Repeat Mode ({})", if repeat_mode { "On" } else { "Off" });
        println!("12. Search and Play a Song");
        println!("13. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                playlist = scan_all_audio_files("C:\\Users\\Ernest\\Desktop\\New folder");
                if playlist.is_empty() {
                    println!("No audio files found.");
                } else {
                    println!("Found {} audio files:", playlist.len());
                    for (i, song) in playlist.iter().enumerate() {
                        println!("{}: {}", i + 1, song);
                    }
                    print!("Enter the number of the song you want to play (or 0 to cancel): ");
                    io::stdout().flush().unwrap();
                    let mut song_choice = String::new();
                    io::stdin().read_line(&mut song_choice).unwrap();
                    if let Ok(index) = song_choice.trim().parse::<usize>() {
                        if index > 0 && index <= playlist.len() {
                            current_index = index - 1;
                            play_song(&playlist[current_index], &sink).await;
                            println!("Now playing: {}", playlist[current_index]);
                        } else {
                            println!("Invalid choice.");
                        }
                    } else {
                        println!("Invalid input.");
                    }
                }
            }
            "2" => {
                if !playlist.is_empty() {
                    play_song(&playlist[current_index], &sink).await;
                } else {
                    println!("No songs in the playlist.");
                }
            }
            "3" => {
                let sink = sink.lock().unwrap();
                sink.pause();
                println!("Paused");
            }
            "4" => {
                let sink = sink.lock().unwrap();
                sink.play();
                println!("Resumed");
            }
            "5" => {
                let sink = sink.lock().unwrap();
                sink.stop();
                println!("Stopped");
            }
            "6" => {
                current_index = next_track_index(&playlist, current_index, shuffle_mode, repeat_mode);
                play_song(&playlist[current_index], &sink).await;
            }
            "7" => {
                current_index = previous_track_index(&playlist, current_index);
                play_song(&playlist[current_index], &sink).await;
            }
            "8" => {
                if !playlist.is_empty() {
                    let song_path = &playlist[current_index];
                    if liked_songs.iter().any(|x| x == song_path) {
                        liked_songs.retain(|x| x != song_path);
                        println!("Unliked song: {}", song_path);
                    } else {
                        liked_songs.push(song_path.clone());
                        println!("Liked song: {}", song_path);
                    }
                } else {
                    println!("No songs in the playlist.");
                }
            }
            "9" => {
                if liked_songs.is_empty() {
                    println!("No liked songs available.");
                } else {
                    println!("Liked Songs:");
                    for (i, song) in liked_songs.iter().enumerate() {
                        println!("{}: {}", i + 1, song);
                    }

                    print!("Enter the number of the song you want to play (or 0 to exit): ");
                    io::stdout().flush().unwrap();
                    let mut song_choice = String::new();
                    io::stdin().read_line(&mut song_choice).unwrap();

                    if let Ok(index) = song_choice.trim().parse::<usize>() {
                        if index > 0 && index <= liked_songs.len() {
                            let song_path = &liked_songs[index - 1];
                            current_index = playlist.iter().position(|x| x == song_path).unwrap_or(0);
                            play_song(song_path, &sink).await;
                            println!("Playing liked song: {}", song_path);
                        } else if index != 0 {
                            println!("Invalid song number.");
                        }
                    } else {
                        println!("Please enter a valid number.");
                    }
                }
            }
            "10" => {
                shuffle_mode = !shuffle_mode;
                println!("Shuffle Mode is now {}", if shuffle_mode { "On" } else { "Off" });
                if shuffle_mode {
                    playlist.shuffle(&mut rand::thread_rng());
                }
            }
            "11" => {
                repeat_mode = !repeat_mode;
                println!("Repeat Mode is now {}", if repeat_mode { "On" } else { "Off" });
            }
            "12" => {
    print!("Enter keyword to search for: ");
    io::stdout().flush().unwrap();
    let mut keyword = String::new();
    io::stdin().read_line(&mut keyword).unwrap();
    let keyword = keyword.trim().to_lowercase();

    let results: Vec<_> = playlist
        .iter()
        .filter(|song| song.to_lowercase().contains(&keyword))
        .collect();

    if results.is_empty() {
        println!("No results found.");
    } else {
        println!("Search results:");
        for (i, song) in results.iter().enumerate() {
            println!("{}: {}", i + 1, song);
        }

        print!("Enter the number of the song you want to play (or 0 to cancel): ");
        io::stdout().flush().unwrap();
        let mut song_choice = String::new();
        io::stdin().read_line(&mut song_choice).unwrap();

        if let Ok(index) = song_choice.trim().parse::<usize>() {
            if index > 0 && index <= results.len() {
                let song_path = results[index - 1];
                current_index = playlist.iter().position(|x| x == song_path).unwrap_or(0);
                play_song(song_path, &sink).await;
                println!("Playing song: {}", song_path);
            } else if index != 0 {
                println!("Invalid song number.");
            }
        } else {
            println!("Please enter a valid number.");
        }
    }
}

            "13" => {
                save_liked_songs(&liked_songs).unwrap();
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}

async fn play_song(song_path: &str, sink: &Arc<Mutex<Sink>>) {
    let file = File::open(song_path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    let mut sink = sink.lock().unwrap();
    sink.stop(); // Stop any existing playback before appending
    sink.append(source);
    sink.play();
    println!("Now playing: {}", song_path);
}

fn next_track_index(playlist: &[String], current_index: usize, shuffle: bool, repeat: bool) -> usize {
    if shuffle {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..playlist.len())
    } else if repeat {
        current_index
    } else if current_index + 1 < playlist.len() {
        current_index + 1
    } else {
        0
    }
}

fn previous_track_index(playlist: &[String], current_index: usize) -> usize {
    if current_index > 0 {
        current_index - 1
    } else {
        playlist.len() - 1
    }
}

fn scan_all_audio_files(directory: &str) -> Vec<String> {
    let mut playlist = Vec::new();
    for entry in WalkDir::new(directory) {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().map(|ext| ext == "mp3" || ext == "wav").unwrap_or(false) {
                playlist.push(path.to_string_lossy().to_string());
            }
        }
    }
    playlist
}

fn load_liked_songs() -> Vec<String> {
    if Path::new("liked_songs.txt").exists() {
        let file = File::open("liked_songs.txt").unwrap();
        let reader = BufReader::new(file);
        reader.lines().filter_map(|line| line.ok()).collect()
    } else {
        vec![]
    }
}

fn save_liked_songs(songs: &[String]) -> io::Result<()> {
    let mut file = File::create("liked_songs.txt")?;
    for song in songs {
        writeln!(file, "{}", song)?;
    }
    Ok(())
}

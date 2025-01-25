// music.js

// Placeholder for song data
let playlist = [];
let currentIndex = 0;

// Load playlist from Rust backend
async function loadPlaylist() {
    // Simulate fetching from Rust backend (use Tauri for real implementation)
    playlist = await fetchPlaylistFromRust();
    const songsList = document.getElementById("songs-list");
    songsList.innerHTML = ""; // Clear existing songs

    playlist.forEach((song, index) => {
        const li = document.createElement("li");
        li.textContent = song;
        li.onclick = () => playSong(index);
        songsList.appendChild(li);
    });
}

// Play song
function playSong(index) {
    currentIndex = index;
    const currentSong = playlist[currentIndex];
    document.getElementById("current-song").textContent = `Now Playing: ${currentSong}`;

    // Simulate playing the song (connect to Rust backend)
    console.log(`Playing song: ${currentSong}`);
}

// Previous song
function prevSong() {
    currentIndex = (currentIndex - 1 + playlist.length) % playlist.length;
    playSong(currentIndex);
}

// Next song
function nextSong() {
    currentIndex = (currentIndex + 1) % playlist.length;
    playSong(currentIndex);
}

// Attach event listeners
document.getElementById("play").addEventListener("click", () => playSong(currentIndex));
document.getElementById("pause").addEventListener("click", () => console.log("Paused"));
document.getElementById("stop").addEventListener("click", () => console.log("Stopped"));
document.getElementById("prev").addEventListener("click", prevSong);
document.getElementById("next").addEventListener("click", nextSong);

// Mock function to simulate Rust backend interaction
async function fetchPlaylistFromRust() {
    // Replace this with Tauri/Rust backend call
    return ["Song 1.mp3", "Song 2.mp3", "Song 3.mp3"];
}

// Initialize the playlist
loadPlaylist();

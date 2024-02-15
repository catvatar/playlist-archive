import { invoke } from "@tauri-apps/api/tauri";

export async function spotifyAPI(url: URL) {
  if (url.pathname.includes("playlist")) {
    return await loadSpotifyPlaylist(getSpotifyPlaylistID(url.href));
  }
  if (url.pathname.includes("track")) {
    return await loadSpotifyTrack(getSpotifyTrackID(url.href));
  }
  return {};
}

async function loadSpotifyTrack(trackID: string) {
  const apiKey = await invoke("get_spotify_api_key");
  const retrived_data = await fetch(
    `https://api.spotify.com/v1/tracks/${trackID}`
  ).then((response) => response.json());
  return retrived_data;
}

async function loadSpotifyPlaylist(playlistID: string) {
  const apiKey = await invoke("get_spotify_api_key");
  const retrived_data = await fetch(
    `https://api.spotify.com/v1/playlists/${playlistID}`,
    {
      headers: {
        Authorization: `Bearer ${apiKey}`,
      },
    }
  ).then((response) => response.json());
  return retrived_data;
}

function getSpotifyTrackID(url: string) {
  const trackId = url.split("track/")[1].split("?")[0];
  return trackId;
}

function getSpotifyPlaylistID(url: string) {
  const playlistId = url.split("playlist/")[1].split("?")[0];
  return playlistId;
}

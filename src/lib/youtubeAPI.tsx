import { invoke } from "@tauri-apps/api/tauri";

export async function youtubeAPI(URL: URL) {
  if (URL.pathname.includes("watch") || URL.href.includes("youtu.be")) {
    return await loadYoutubeVideo(getYoutubeVideoID(URL.href));
  }
  if (URL.pathname.includes("playlist")) {
    return await loadYoutubePlaylist(getYoutubePlaylistID(URL.href));
  }
  return {};
}

// input: URL: string
// output: raw_data: any|{}
export async function loadYoutubeVideo(videoID: string) {
  const apiKey = await invoke("get_youtube_api_key");

  const retrived_data = await fetch(
    `https://youtube.googleapis.com/youtube/v3/videos?part=snippet&id=${videoID}&key=${apiKey}`
  ).then((response) => response.json());
  return retrived_data;
}

// input: URL: string
// output: raw_data: any|{}
export async function loadYoutubePlaylist(playlistID: string) {
  const apiKey = await invoke("get_youtube_api_key");

  const retrived_data = await fetch(
    `https://youtube.googleapis.com/youtube/v3/playlistItems?part=snippet%2CcontentDetails&maxResults=50&playlistId=${playlistID}&key=${apiKey}`
  ).then((response) => response.json());
  return retrived_data;
}

function getYoutubeVideoID(URL: string) {
  if (URL.includes("youtu.be")) {
    const videoId = URL.split("youtu.be/")[1].split("?")[0];
    return videoId;
  }
  const videoId = URL.split("v=")[1];
  const ampersandPosition = videoId.indexOf("&");
  if (ampersandPosition !== -1) {
    return videoId.substring(0, ampersandPosition);
  }
  return videoId;
}

function getYoutubePlaylistID(URL: string) {
  const playlistId = URL.split("list=")[1].split("?")[0];
  return playlistId;
}

//&pageToken=${pageToken}

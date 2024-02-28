import { invoke } from "@tauri-apps/api/tauri";
// input: source: string
// output: {title: string, artist: string, album: string, year: string, genre: string[], tags: string[], sources: string[]} | {}
export default async function loadMedia(
  url: string,
  forcedSource: string = ""
) {
  // creating URL from string
  if (URL.canParse(url) === false) return "Error, ,could not parse URL";
  const urlObject: URL = new URL(url);

  // matching hostname to source
  const source = urlObject.hostname;
  if (forcedSource === "YouTube" || hostIsYouTube(source)) {
    return handleYoutubeVideo(urlObject);
  }

  if (forcedSource === "Spotify" || hostIsSpotify(source)) {
    return "Error, Spotify, not implemented";
  }

  return "Error, ,could not find source";
}

async function handleYoutubeVideo(url: URL) {
  const params = url.searchParams;
  const source = url.hostname;

  if (source === "youtu.be") {
    const id = url.pathname.substring(1);
    const response = await invoke("get_video_from_youtube_by_id", { id });
    // console.log(response);
    return "Success, Youtube, video";
  }
  if (params.has("v")) {
    const id = params.get("v");
    const response = await invoke("get_video_from_youtube_by_id", { id });
    // console.log(response);
    return "Success, YouTube, video";
  }
  if (params.has("list")) {
    const id = params.get("list");
    const response = await invoke("get_videos_from_youtube_by_playlist_id", {
      id,
    });
    console.log(response);
    return "Success, YouTube, playlist";
  }
  return "Error, YouTube, could not find video or playlist id";
}

function hostIsSpotify(host: string) {
  return (
    host === "www.spotify.com" ||
    host === "spotify.com" ||
    host === "open.spotify.com"
  );
}

function hostIsYouTube(host: string) {
  return (
    host === "www.youtube.com" ||
    host === "youtube.com" ||
    host === "youtu.be" ||
    host === "music.youtube.com"
  );
}

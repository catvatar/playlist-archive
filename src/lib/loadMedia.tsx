import { invoke } from "@tauri-apps/api/tauri";
// input: source: string
// output: {title: string, artist: string, album: string, year: string, genre: string[], tags: string[], sources: string[]} | {}
export default async function loadMedia(url: string) {
  try {
    new URL(url);
  } catch (error) {
    console.error("Error parsing URL:", error);
    return {};
  }
  const urlObject = new URL(url);
  const source = urlObject.hostname;
  const id = idFromYouTube(url);
  const response = await invoke("get_video_from_youtube_by_id", { source, id });
  if (response) {
    return response;
  }
  return {};
}
function idFromYouTube(url: string) {
  const urlObject = new URL(url);
  return urlObject.searchParams.get("v");
}

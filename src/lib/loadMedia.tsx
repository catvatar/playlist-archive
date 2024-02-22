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
  console.log(invoke("get_video_from_youtube_by_id", { id: "123" }));
  return {};
}

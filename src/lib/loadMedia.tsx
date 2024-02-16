// import { printYoutubeVideo } from "./youtubeAPI";
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
  // if (source === "www.youtube.com" || source === "www.youtu.be") {
  //   return youtubeInterface(urlObject);
  // }
  return {};
}

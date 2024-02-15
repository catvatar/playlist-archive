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
  switch (source) {
    case "www.youtube.com":
      return {};
    case "youtu.be":
      return {};
    case "open.spotify.com":
      return {};
    default:
      console.error("Load Media: Unsupported source.");
      return {};
  }
}

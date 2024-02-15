// input: retrived_data: {source: string, raw_data: any}
// output: {title: string, artist: string, album: string, year: string, genre: string[], tags: string[], sources: string[]}
export default function dataInterface(retrived_data: any) {
  switch (retrived_data.source) {
    case "youtube":
      return youtubeSongInterface(retrived_data.raw_data);
    default:
      return {};
  }
}

function youtubeSongInterface(raw_data: any) {
  const data = {
    title: raw_data.items[0].snippet.title,
    artist: raw_data.items[0].snippet.channelTitle,
    album: "",
    year: raw_data.items[0].snippet.publishedAt.split("-")[0],
    genre: [...raw_data.items[0].snippet.tags],
    tags: [],
    sources: [`https://www.youtube.com/watch?v=${raw_data.items[0].id}`],
  };
  return data;
}

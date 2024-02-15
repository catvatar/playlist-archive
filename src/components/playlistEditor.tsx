"use client";
import { useRouter } from "next/navigation";
import React, { useState } from "react";

interface Song {
  name: string;
}

interface Playlist {
  name: string;
  songs?: Song[];
}

export default function PlaylistTreeView({ data }: any) {
  const [expandedPlaylists, setExpandedPlaylists] = useState<string[]>([]);
  const router = useRouter();

  const togglePlaylist = (playlistId: string) => {
    if (expandedPlaylists.includes(playlistId)) {
      setExpandedPlaylists(
        expandedPlaylists.filter((name) => name !== playlistId)
      );
    } else {
      setExpandedPlaylists([...expandedPlaylists, playlistId]);
    }
  };

  const renderPlaylist = (playlist: Playlist) => {
    const isExpanded = expandedPlaylists.includes(playlist.name);

    return (
      <ul>
        <li>
          <h2 onClick={() => togglePlaylist(playlist.name)}>
            {(isExpanded ? "- " : "+ ") + playlist.name}
          </h2>
        </li>
        {isExpanded && playlist.songs && (
          <ul>
            {playlist.songs.map((song) => (
              <li className="p-1 pl-6" key={song.name}>
                {song.name}
              </li>
            ))}
          </ul>
        )}
      </ul>
    );
  };

  return (
    <ul>
      {data.map((playlist: any) => (
        <li className="flex items-stretch" key={playlist.name}>
          <div className="p-2 pl-6 text-start flex flex-col">
            {renderPlaylist(playlist)}
          </div>
          <button
            className="p-2 pl-6 self-start"
            onClick={() => router.push(`/playlist-editor/${playlist.name}`)}
          >
            <h2>Edit</h2>
          </button>
        </li>
      ))}
    </ul>
  );
}

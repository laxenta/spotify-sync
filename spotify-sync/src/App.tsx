import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface Track {
  id: string;
  name: string;
  artists: string[];
  album: string;
  uri: string;
}

export default function App() {
  const [fromTracks, setFromTracks] = useState<Track[]>([]);
  const [toTracks, setToTracks] = useState<Track[]>([]);
  const [fromAuthenticated, setFromAuthenticated] = useState(false);
  const [toAuthenticated, setToAuthenticated] = useState(false);
  const [loading, setLoading] = useState(false);
  const [transferring, setTransferring] = useState(false);

  useEffect(() => {
    loadSavedTokens();
  }, []);

  const loadSavedTokens = async () => {
    try {
      // Check if invoke is available (it won't be in Codespaces)
      if (typeof invoke !== "function") {
        console.log("Tauri invoke not available (Codespaces environment)");
        return;
      }
      const saved = await invoke("load_saved_tokens");
      if (saved && typeof saved === "object") {
        const savedObj = saved as Record<string, boolean>;
        if (savedObj.from) setFromAuthenticated(true);
        if (savedObj.to) setToAuthenticated(true);
      }
    } catch (error) {
      console.error("Failed to load saved tokens:", error);
    }
  };

  const handleLogin = async (panel: "from" | "to") => {
    try {
      if (typeof invoke !== "function") {
        alert("Tauri not available in Codespaces. Clone locally to test.");
        return;
      }
      const url: string = await invoke("get_oauth_url", { panel });
      window.open(url, "SpotifyAuth", "width=500,height=700");

      // Poll to check if authentication was successful
      const checkAuth = setInterval(async () => {
        try {
          const saved = await invoke("load_saved_tokens");
          if (saved && typeof saved === "object") {
            const savedObj = saved as Record<string, boolean>;
            if (panel === "from" && savedObj.from) {
              setFromAuthenticated(true);
              clearInterval(checkAuth);
            } else if (panel === "to" && savedObj.to) {
              setToAuthenticated(true);
              clearInterval(checkAuth);
            }
          }
        } catch (error) {
          console.error("Auth check failed:", error);
        }
      }, 1000);

      setTimeout(() => clearInterval(checkAuth), 60000); // Stop checking after 1 minute
    } catch (error) {
      console.error("Login failed:", error);
      alert("Login failed. Check console for details.");
    }
  };

  const handleFetchSongs = async (panel: "from" | "to") => {
    setLoading(true);
    try {
      const tracks: Track[] = await invoke("fetch_liked_songs", { panel });

      if (panel === "from") {
        setFromTracks(tracks);
      } else {
        setToTracks(tracks);
      }
    } catch (error) {
      console.error("Failed to fetch songs:", error);
      alert(`Failed to fetch songs: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const handleTransfer = async () => {
    if (fromTracks.length === 0) {
      alert("No songs to transfer!");
      return;
    }

    if (!toAuthenticated) {
      alert("Please authenticate the target account first!");
      return;
    }

    setTransferring(true);
    try {
      await invoke("transfer_tracks");
      alert("✅ Transfer complete!");
      await handleFetchSongs("to");
    } catch (error) {
      console.error("Transfer failed:", error);
      alert(`Transfer failed: ${error}`);
    } finally {
      setTransferring(false);
    }
  };

  return (
    <div className="app-container">
      <header className="app-header">
        <h1>🎵 Spotify Sync</h1>
        <p>Transfer your music between accounts</p>
      </header>

      <div className="main-grid">
        {/* FROM PANEL */}
        <div className="panel from-panel">
          <div className="panel-header">
            <h2>FROM</h2>
            {fromAuthenticated && <span className="badge">✓ Connected</span>}
          </div>

          <div className="panel-body">
            {!fromAuthenticated ? (
              <button
                className="btn btn-primary"
                onClick={() => handleLogin("from")}
              >
                Login with Spotify
              </button>
            ) : (
              <>
                <button
                  className="btn btn-secondary"
                  onClick={() => handleFetchSongs("from")}
                  disabled={loading}
                >
                  {loading ? "Loading..." : "Fetch Liked Songs"}
                </button>

                {fromTracks.length > 0 && (
                  <div className="song-count">
                    {fromTracks.length} songs loaded
                  </div>
                )}

                <div className="track-list">
                  {fromTracks.length > 0 ? (
                    fromTracks.map((track) => (
                      <div key={track.id} className="track-item">
                        <div className="track-info">
                          <div className="track-name">{track.name}</div>
                          <div className="track-artist">
                            {track.artists.join(", ")}
                          </div>
                          <div className="track-album">{track.album}</div>
                        </div>
                      </div>
                    ))
                  ) : (
                    <div className="placeholder">
                      {fromAuthenticated
                        ? "Click 'Fetch Liked Songs' to load your music"
                        : "Login to see your songs"}
                    </div>
                  )}
                </div>
              </>
            )}
          </div>
        </div>

        {/* PREVIEW PANEL */}
        <div className="panel preview-panel">
          <div className="panel-header">
            <h2>PREVIEW</h2>
          </div>

          <div className="panel-body preview-body">
            <div className="preview-list">
              {fromTracks.length > 0 ? (
                <>
                  {fromTracks.slice(0, 50).map((track) => (
                    <div key={track.id} className="preview-track">
                      <div className="preview-number">
                        {fromTracks.indexOf(track) + 1}
                      </div>
                      <div className="preview-info">
                        <div className="preview-name">{track.name}</div>
                        <div className="preview-artist">{track.artists[0]}</div>
                      </div>
                    </div>
                  ))}
                </>
              ) : (
                <div className="placeholder">
                  Select a source account to preview songs
                </div>
              )}
            </div>

            <div className="transfer-section">
              <div className="transfer-stats">
                <div className="stat-box">
                  <div className="stat-label">Ready to Transfer</div>
                  <div className="stat-value">{fromTracks.length}</div>
                </div>
              </div>

              <button
                className="btn btn-transfer"
                onClick={handleTransfer}
                disabled={
                  fromTracks.length === 0 || !toAuthenticated || transferring
                }
              >
                {transferring ? "Transferring..." : "TRANSFER"}
              </button>
            </div>
          </div>
        </div>

        {/* TO PANEL */}
        <div className="panel to-panel">
          <div className="panel-header">
            <h2>TO</h2>
            {toAuthenticated && <span className="badge">✓ Connected</span>}
          </div>

          <div className="panel-body">
            {!toAuthenticated ? (
              <button
                className="btn btn-primary"
                onClick={() => handleLogin("to")}
              >
                Login with Spotify
              </button>
            ) : (
              <>
                <button
                  className="btn btn-secondary"
                  onClick={() => handleFetchSongs("to")}
                  disabled={loading}
                >
                  {loading ? "Loading..." : "Fetch Liked Songs"}
                </button>

                {toTracks.length > 0 && (
                  <div className="song-count">
                    {toTracks.length} songs loaded
                  </div>
                )}

                <div className="track-list">
                  {toTracks.length > 0 ? (
                    toTracks.map((track) => (
                      <div key={track.id} className="track-item">
                        <div className="track-info">
                          <div className="track-name">{track.name}</div>
                          <div className="track-artist">
                            {track.artists.join(", ")}
                          </div>
                          <div className="track-album">{track.album}</div>
                        </div>
                      </div>
                    ))
                  ) : (
                    <div className="placeholder">
                      {toAuthenticated
                        ? "Click 'Fetch Liked Songs' to load your music"
                        : "Login to see your songs"}
                    </div>
                  )}
                </div>
              </>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
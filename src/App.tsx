import { useState } from "react";

import "./App.css";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [url, setUrl] = useState("");
  // const handleGreet = async () => {
  //   let result: any = await invoke("greet", { name: "katte" });

  //   console.info(result);
  // };

  const handleCamera = async () => {
    setUrl("");
    await invoke("server");
    setUrl("http://127.0.0.1:8000/");
  };
  return (
    <>
      <div style={{ display: "flex" }}>
        <div style={{ width: "50%", padding: "10px" }}>
          <div style={{ display: "flex", alignItems: "center" }}>
            <span>姓名</span>
            <input
              type="text"
              style={{ width: 200, height: 30, marginLeft: 10, fontSize: 20 }}
            />
          </div>
          <div style={{ display: "flex", alignItems: "center", marginTop: 40 }}>
            <span>学号</span>
            <input
              type="text"
              style={{ width: 200, height: 30, marginLeft: 10, fontSize: 20 }}
            />
          </div>
          <div style={{ marginTop: 40, display: "flex", alignItems: "center" }}>
            <button onClick={handleCamera}>摄像头</button>
          </div>
        </div>
        <div style={{ flex: "1", marginLeft: 60 }}>
          <img
            src={url}
            alt="摄像头人像"
            style={{
              width: 350,
              height: 500,
              objectFit: "cover",
              background: "#3d86ef",
            }}
          />
        </div>
      </div>
    </>
  );
}

export default App;

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

document.addEventListener("DOMContentLoaded", () => {
  const xPosInput = document.getElementById("x-pos") as HTMLInputElement | null;
  const yPosInput = document.getElementById("y-pos") as HTMLInputElement | null;
  const intervalInput = document.getElementById(
    "interval"
  ) as HTMLInputElement | null;
  const startBtn = document.getElementById(
    "start-btn"
  ) as HTMLButtonElement | null;
  const stopBtn = document.getElementById(
    "stop-btn"
  ) as HTMLButtonElement | null;
  const getPosBtn = document.getElementById(
    "get-position"
  ) as HTMLButtonElement | null;
  const statusIndicator = document.getElementById(
    "status-indicator"
  ) as HTMLElement | null;
  const statusText = document.getElementById(
    "status-text"
  ) as HTMLElement | null;

  // Make sure all required elements exist
  if (
    !xPosInput ||
    !yPosInput ||
    !intervalInput ||
    !startBtn ||
    !stopBtn ||
    !getPosBtn ||
    !statusIndicator ||
    !statusText
  ) {
    console.error("Missing one or more required DOM elements.");
    return;
  }

  // Event listener for the current position button
  getPosBtn.addEventListener("click", async () => {
    try {
      const position = await invoke<[number, number]>(
        "get_current_mouse_position"
      );
      xPosInput.value = position[0].toString();
      yPosInput.value = position[1].toString();
    } catch (error) {
      console.error("Failed to get mouse position:", error);
    }
  });

  // Event listener for the start button
  startBtn.addEventListener("click", async () => {
    const x = parseInt(xPosInput.value, 10);
    const y = parseInt(yPosInput.value, 10);
    const interval = parseInt(intervalInput.value, 10);

    if (isNaN(x) || isNaN(y) || isNaN(interval) || interval < 100) {
      alert("กรุณาใส่ค่าที่ถูกต้อง (ระยะเวลาต้องมากกว่า 100ms)");
      return;
    }

    try {
      await invoke("start_clicking", { x, y, intervalMs: interval });
      startBtn.disabled = true;
      stopBtn.disabled = false;
      updateStatus(true);
    } catch (error) {
      console.error("Failed to start clicking:", error);
    }
  });

  // Event listener for the stop button
  stopBtn.addEventListener("click", async () => {
    try {
      await invoke("stop_clicking");
      startBtn.disabled = false;
      stopBtn.disabled = true;
      updateStatus(false);
    } catch (error) {
      console.error("Failed to stop clicking:", error);
    }
  });
  listen("shortcut-event", (event: { payload: string }) => {
    if (event.payload === "Ctrl-N Released!") {
      stopBtn.click();
    }
  });

  // ESC key to stop clicking
  document.addEventListener("keydown", async (event) => {
    if (event.key === "Escape") {
      try {
        await invoke("stop_clicking");
        startBtn.disabled = false;
        stopBtn.disabled = true;
        updateStatus(false);
      } catch (error) {
        console.error("Failed to stop clicking:", error);
      }
    }
  });
  document.addEventListener("keydown", async (event) => {
    if (event.ctrlKey&& event.key === "s") {
      startBtn.click();
    }
  });
  // Ctrl + Shift + C to enable current position
  document.addEventListener("keydown", async (event) => {
    if (event.key === "x") {
      getPosBtn.click();
    }
  });

  // Listen for clicking status updates
  listen("clicking-status", (event: { payload: boolean }) => {
    updateStatus(event.payload);
  });

  // Function to update the status indicator
  function updateStatus(isActive: boolean) {
    if (!statusIndicator || !statusText || !startBtn || !stopBtn) return;

    if (isActive) {
      statusIndicator.classList.add("active");
      statusText.textContent = "กำลังทำงาน";
    } else {
      statusIndicator.classList.remove("active");
      statusText.textContent = "ไม่ได้ทำงาน";
      startBtn.disabled = false;
      stopBtn.disabled = true;
    }
  }
});

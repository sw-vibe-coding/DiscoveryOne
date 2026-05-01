import * as THREE from "https://cdn.jsdelivr.net/npm/three@0.164.1/build/three.module.js";

const FACE_DEPTH = {
  Front: 0,
  Left: -8,
  Right: 8,
  Top: 16,
  Bottom: -16,
  Rear: 24,
  Internal: 32,
};

export function initDiscoveryOne3dViewer(mount, symbolsJson) {
  const data = typeof symbolsJson === "string" ? JSON.parse(symbolsJson) : symbolsJson;
  window.discoveryOne3dSymbols = data;
  const scene = new THREE.Scene();
  scene.name = `DiscoveryOne ${data.definition}`;
  const camera = new THREE.PerspectiveCamera(45, 1, 0.1, 1000);
  camera.position.set(24, 22, 68);
  camera.lookAt(8, -2, 8);

  const renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true });
  renderer.setSize(480, 300, false);
  mount.replaceChildren(renderer.domElement);

  const material = new THREE.MeshBasicMaterial({ color: 0x2f6f73 });
  const geometry = new THREE.BoxGeometry(0.9, 0.45, 0.2);
  for (const symbol of data.symbols) {
    const block = new THREE.Mesh(geometry, material.clone());
    block.name = `symbol:${symbol.face}:${symbol.text}`;
    block.position.set(symbol.x, -symbol.y, (FACE_DEPTH[symbol.face] ?? 0) + symbol.z);
    scene.add(block);
  }

  renderer.render(scene, camera);
  mount.dataset.sceneReady = "true";
  mount.dataset.symbolCount = String(data.symbol_count);
  mount.dataset.threeRevision = THREE.REVISION;
}

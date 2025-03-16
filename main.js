// main.js
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import init, { Simulation } from './pkg/hylaean_path.js';

async function run() {
  // Initialize the Wasm module.
  await init();

  // Create a simulation with 10 satellites.
  const sim = new Simulation(10);

  // Set up three.js scene.
  const scene = new THREE.Scene();

  // Set up a camera.
  const camera = new THREE.PerspectiveCamera(
    75,
    window.innerWidth / window.innerHeight,
    0.1,
    1e9
  );
  camera.position.set(1e7, 1e7, 1e7); // Adjust as needed

  // Create the renderer.
  const renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setSize(window.innerWidth, window.innerHeight);
  document.body.appendChild(renderer.domElement);

  // Add OrbitControls to allow user to orbit around the scene.
  const controls = new OrbitControls(camera, renderer.domElement);
  controls.target.set(0, 0, 0);
  controls.update();

  // Create an array to hold satellite mesh objects.
  const satelliteMeshes = [];
  const geometry = new THREE.SphereGeometry(1e5, 16, 16);
  const material = new THREE.MeshBasicMaterial({ color: 0xff0000 });
  for (let i = 0; i < 10; i++) {
    const mesh = new THREE.Mesh(geometry, material);
    scene.add(mesh);
    satelliteMeshes.push(mesh);
  }

  // Animation loop.
  function animate() {
    requestAnimationFrame(animate);

    // Step the simulation.
    sim.step();

    // Retrieve updated satellite positions.
    const positions = sim.get_positions(); // Returns an array of [x, y, z] arrays.
    positions.forEach((pos, i) => {
      if (satelliteMeshes[i]) {
        satelliteMeshes[i].position.set(pos[0], pos[1], pos[2]);
      }
    });

    // Update orbit controls.
    controls.update();

    // Render the scene.
    renderer.render(scene, camera);
  }
  animate();
}

run();

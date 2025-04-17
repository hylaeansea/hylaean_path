// main.js
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls.js';
import init, { Simulation } from './pkg/hylaean_path.js';

async function run() {
  // Initialize the Wasm module.
  await init();

  // Set the number of satellites as a variable.
  const nSatellites = 5000;
  // Create a simulation with nSatellites.
  const sim = new Simulation(nSatellites);

  // Set up three.js scene.
  const scene = new THREE.Scene();

  // Add axis helper (length can be adjusted; here we use 1e7 for visibility)
  const axesHelper = new THREE.AxesHelper(1e7);
  scene.add(axesHelper);

  // Set up a camera.
  const camera = new THREE.PerspectiveCamera(
    75,
    window.innerWidth / window.innerHeight,
    0.1,
    1e9
  );
  camera.position.set(1e7, 1e7, 1e7);

  // Create the renderer.
  const renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setSize(window.innerWidth, window.innerHeight);
  document.body.appendChild(renderer.domElement);

  // Add OrbitControls.
  const controls = new OrbitControls(camera, renderer.domElement);
  controls.target.set(0, 0, 0);
  controls.update();

  // Add a wireframe icosahedron with a radius of 6,700,000 meters (solid version)
  const icosahedronGeometry = new THREE.IcosahedronGeometry(6700000, 4);
  icosahedronGeometry.computeVertexNormals(); // Recompute normals
  
  const icosahedronMaterial = new THREE.MeshBasicMaterial({
    color: 0x888888,
    wireframe: false,
    side: THREE.FrontSide,
    depthTest: true,
    depthWrite: true,
  });
  const icosahedronMesh = new THREE.Mesh(icosahedronGeometry, icosahedronMaterial);
  icosahedronMesh.renderOrder = 0;
  scene.add(icosahedronMesh);
  
  // When creating satellite meshes:
  const satelliteMeshes = [];
  const satelliteGeometry = new THREE.SphereGeometry(1e5, 16, 16);
  const satelliteMaterial = new THREE.MeshBasicMaterial({ color: 0xff0000 });
  for (let i = 0; i < nSatellites; i++) {
    const mesh = new THREE.Mesh(satelliteGeometry, satelliteMaterial);
    mesh.renderOrder = 1; // Render after the icosahedron
    scene.add(mesh);
    satelliteMeshes.push(mesh);
  }


  // Animation loop.
  function animate() {
    requestAnimationFrame(animate);

    // Step the simulation.
    sim.step();

    // Update satellite positions.
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

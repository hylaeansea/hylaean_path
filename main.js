// main.js
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';


import init, { Simulation } from './pkg/hylaean_path.js';

async function run() {
  // Initialize the Wasm module.
  await init();

  // Set the number of satellites as a variable.
  const nSatellites = 500;
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
  const renderer = new THREE.WebGLRenderer({ logarithmicDepthBuffer: true, antialias: true });
  renderer.setSize(window.innerWidth, window.innerHeight);
  document.body.appendChild(renderer.domElement);

  // Add OrbitControls.
  const controls = new OrbitControls(camera, renderer.domElement);
  controls.target.set(0, 0, 0);
  controls.update();

  //— LIGHTS —//
  const ambientLight = new THREE.AmbientLight(0x404040, 1.0); // soft white ambient
  scene.add(ambientLight);

  const dirLight = new THREE.DirectionalLight(0xffffff, 0.8);
  dirLight.position.set(1e7, 1e7, 1e7).normalize();
  scene.add(dirLight);

  //— ICOSAHEDRON —//
  const icoGeo = new THREE.IcosahedronGeometry(6_700_000, 2);
  icoGeo.computeVertexNormals(); // recompute normals especially if you’ve inverted or scaled

  const icoMat = new THREE.MeshStandardMaterial({
    color: 0x88ffff,
    side: THREE.FrontSide,  // render only front faces
    metalness: 0.0,
    roughness: 1.0,
    transparent: false,     // fully opaque
    depthTest: true,
    depthWrite: true,
  });

  const icoMesh = new THREE.Mesh(icoGeo, icoMat);
  scene.add(icoMesh);
  
  // When creating satellite meshes:
  const satelliteMeshes = [];
  const satelliteGeometry = new THREE.SphereGeometry(1e5, 16, 16);
  
  for (let i = 0; i < nSatellites; i++) {
    const satelliteMaterial  = new THREE.MeshStandardMaterial({ color: 0xff0000 });
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

      // Get satellites in proximity warning state
      const proximityWarnings = sim.get_proximity_warnings(); // Get the array of satellite IDs in proximity
      console.log(proximityWarnings);
      // Update satellite positions and colors.
      const positions = sim.get_positions(); // Returns an array of [x, y, z] arrays.
      positions.forEach((pos, i) => {
        if (satelliteMeshes[i]) {
          // Update position
          satelliteMeshes[i].position.set(pos[0], pos[1], pos[2]);
          
          // Update color based on proximity warning
          if (proximityWarnings.includes(i)) {
            // Set to bright red for satellites in proximity
            satelliteMeshes[i].material.color.setRGB(1, 0, 0);
          } else {
            // Set to white (or any default color) for normal satellites
            satelliteMeshes[i].material.color.setRGB(1, 1, 1);
          }
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

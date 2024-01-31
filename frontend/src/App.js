import React, { useState } from 'react';
import './k.css';
import MapComponent2 from './components/MapComponent';
import Modal from './components/Modal'; // Assuming you have a Modal component
import TemplateEditor from './components/TemplateEditor'; // Assuming you have a TemplateEditor component

function App() {
  let [boundingBox, setBoundingBox] = useState({
    northWest: { lat: "", long: "" },
    southEast: { lat: "", long: "" }
  });

  const [isModalOpen, setModalOpen] = useState(false);

  const handleOpenModal = () => {
    setModalOpen(true);
  };

  const handleCloseModal = () => {
    setModalOpen(false);
  };

  return (
    <div className="App bg-thales-dark min-h-screen flex flex-col">
      {/* Map Area */}
      <div className="swn">
        <span className="py-2 text-white font-bold text-xl md:text-4xl">
          Geo<span className="text-green-500">Query</span>
        </span>
      </div>
      <div>
        <MapComponent2 setBoundingBox={setBoundingBox} boundingBox={boundingBox}/>
      </div>

      {/* Button to Open Modal */}
      <button 
        onClick={handleOpenModal} 
        className="bg-green-500 text-white font-bold py-2 px-4 rounded"
      >
        Open Template Editor
      </button>

      {/* Modal for Template Editor */}
      <Modal isOpen={isModalOpen} onClose={handleCloseModal}>
        <TemplateEditor />
      </Modal>
    </div>
  );
}

export default App;

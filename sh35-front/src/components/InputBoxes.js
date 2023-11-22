import React, { useState } from 'react';

const LongLatBoxes = () => {
  const [input1, setInput1] = useState('');
  const [input2, setInput2] = useState('');

  const handleInputChange1 = (e) => {
    setInput1(e.target.value);
  };

  const handleInputChange2 = (e) => {
    setInput2(e.target.value);
  };

  return (
    <div>
      <input
        type="text"
        placeholder="Long"
        value={input1}
        onChange={handleInputChange1}
      />
      <input
        type="text"
        placeholder="Lat"
        value={input2}
        onChange={handleInputChange2}
      />
    </div>
  );
};

export default LongLatBoxes;

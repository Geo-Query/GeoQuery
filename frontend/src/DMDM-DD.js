const Decimal = require('decimal.js');

function dmdmToDecimal(degrees, minutes, decimalMinutes, direction) {
    const decimalDegrees = new Decimal(degrees)
    .plus(new Decimal(minutes).dividedBy(60))
    .plus(new Decimal(decimalMinutes).dividedBy(60));

    // Adjust for the direction (N, S, E, W)
    if (direction === 'S' || direction === 'W') {
    return decimalDegrees.negated();
  }

  return decimalDegrees;
}

// Example of Latitude: 35° 15.4567' N, Longitude: 100° 30.6789' W
const latitude = dmdmToDecimal(35, 15, 0.4567, 'N');
const longitude = dmdmToDecimal(100, 30, 0.6789, 'W');

// In the above example. Output should be Decimal Latitude: 35.2576, Decimal Longitude: -100.5113
console.log('Latitude:', latitude.toString());
console.log('Longitude:', longitude.toString());
const Decimal = require('decimal.js');

function dmsToDecimal(degrees, minutes, seconds, direction) {
  const decimalDegrees = new Decimal(degrees)
    .plus(new Decimal(minutes).dividedBy(60))
    .plus(new Decimal(seconds).dividedBy(3600));

  // Adjust for the direction (N, S, E, W)
  if (direction === 'S' || direction === 'W') {
    return decimalDegrees.negated();
  }

  return decimalDegrees;
}

// Example of Latitude: 40° 42' 51" N, Longitude: 74° 0' 21" W
const latitude = dmsToDecimal(40, 42, 51, 'N');
const longitude = dmsToDecimal(74, 0, 21, 'W');

// In the above example. Output should be Decimal Latitude: 40.714167, Decimal Longitude: -74.005833
console.log('Latitude:', latitude.toString());
console.log('Longitude:', longitude.toString());
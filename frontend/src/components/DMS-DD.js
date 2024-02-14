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

// export default dmsToDecimal;

// Example of Latitude: 40° 42' 51" N, Longitude: 74° 0' 21" W
// const latitude = dmsToDecimal(40, 42, 51, 'N');
// const longitude = dmsToDecimal(74, 0, 21, 'W');

// In the above example. Output should be Decimal Latitude: 40.714167, Decimal Longitude: -74.005833
// console.log('Latitude:', latitude.toString());
// console.log('Longitude:', longitude.toString());

// Regex and example to satisfy
const DMS = /^(\d{1,2} \d{1,2} \d{1,2}) ([NESW])$/;
const lat = "40 42 51 N";

console.log(DMS.test(lat));

const digits = lat.split(' ');
console.log(digits);
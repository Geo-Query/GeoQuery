const Decimal = require('decimal.js');

function dmdmToDecimal(degrees, minutes, direction) {
    const decimalDegrees = new Decimal(degrees)
        .plus(new Decimal(minutes).dividedBy(60))

    // Adjust for the direction (N, S, E, W)
    if (direction === 'S' || direction === 'W') {
    return decimalDegrees.negated();
  }

  return decimalDegrees;
}

// export default dmdmToDecimal;

// Example of Latitude: 35° 15.4567' N, Longitude: 100° 30.6789' W
// const latitude = dmdmToDecimal(35, 15, 0.4567, 'N');
// const longitude = dmdmToDecimal(100, 30, 0.6789, 'W');

// In the above example. Output should be Decimal Latitude: 35.2576, Decimal Longitude: -100.5113
// console.log('Latitude:', latitude.toString());
// console.log('Longitude:', longitude.toString());

// Regex and example to satisfy
const DMDM = /^(\d{1,2} \d{1,2}\.\d{4}) ([NESW])$/;
let lat = "35 15.4567 N";

console.log(DMDM.test(lat));

const digits = lat.split(' ');
console.log(digits);
console.log(dmdmToDecimal(digits[0], digits[1], digits[2]));
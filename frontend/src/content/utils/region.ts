import dmsToDecimal from "../components/DMS-DD";
import dmdmToDecimal from "../components/DMDM-DD";

export interface Coordinate {
    lat: number,
    long: number,
}

export interface Region {
    northWest: Coordinate,
    southEast: Coordinate,
}

export default class SelectedRegion {
    enteredManually: boolean;
    region?: Region;

    constructor(enteredManually: boolean, region?: Region) {
        this.enteredManually = enteredManually;
        this.region = region;
    }
}

export function validateAndConformCoordinate(initial: string): number {
    return parseFloat(initial);
}

export function convertAndOrValidate(initial: string): number {

    const DD = /^\\d*\\.?\\d+$/;
    const DMS = /^(\d{1,2} \d{1,2} \d{1,2}) ([NESW])$/;
    const DMDM = /^(\d{1,2} \d{1,2}\.\d{4}) ([NESW])$/;

    if(DMS.test(initial)){
        const values = initial.split(" ");
        return dmsToDecimal(parseInt(values[0]), parseInt(values[1]), parseInt(values[2]), values[3]);
    } else if (DMDM.test(initial)){
        const values = initial.split(" ");
        return dmdmToDecimal(parseInt(values[0]), parseFloat(values[1]), values[2]);
    } else if (DD.test(initial)) {
        return parseFloat(initial);
    } else {
        return 999; //if the box entry is not in either DD, DMS or DMDM return error code 999
    }
}

// export function checkFormat(initial: string): number {
//
//     const DMS = /^(\d{1,2} \d{1,2} \d{1,2}) ([NESW])$/;
//     const DMDM = /^(\d{1,2} \d{1,2}\.\d{4}) ([NESW])$/;
//
//     const isDMS = (DMS.test(initial));
//     const isDMDM = (DMDM.test(initial));
//
//     if(isDMS){
//         const digits = initial.split(" ");
//         return(dmsToDecimal(parseFloat(digits[0]), parseFloat(digits[1]), parseFloat(digits[2]), digits[3]));
//     } else if (isDMDM){
//         const digits = initial.split(" ");
//         return(dmdmToDecimal(parseFloat(digits[0]), parseFloat(digits[1]), digits[2]));
//     } else {
//         return parseFloat(initial);
//     }
// }
//
// export function checkValid(coordType: 'lat' | 'long', initial:  number): { isValid: boolean; result?: number; error?: string} {
//
//     switch (coordType) {
//         case 'lat':
//             if (initial <= 90 && initial >= -90) {
//                 return { isValid: true, result: initial};
//             } else {
//                 return { isValid: false, error: 'Invalid Latitude'};
//             }
//         case 'long':
//             if (initial <= 180 && initial >= -180) {
//                 return { isValid: true, result: initial };
//             } else {
//                 return { isValid: false, error: 'Invalid Longitude' };
//             }
//         default:
//             return { isValid: false, error: 'Invalid coordinate type' };
//     }
// }
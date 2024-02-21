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
    region?: Region;

    constructor(region?: Region) {
        this.region = region;
    }
}

export function validateAndConformCoordinate(initial: string): number {
    return parseFloat(initial);
}

export function checkFormat(initial: string): string {

    const DMS = /^(\d{1,2} \d{1,2} \d{1,2}) ([NESW])$/;
    const DMDM = /^(\d{1,2} \d{1,2}\.\d{4}) ([NESW])$/;

    const isDMS = (DMS.test(initial));
    const isDMDM = (DMDM.test(initial));

    if(isDMS){
        const digits = initial.split(" ");
        return(dmsToDecimal(parseFloat(digits[0]), parseFloat(digits[1]), parseFloat(digits[2]), digits[3]));
    } else if (isDMDM){
        const digits = initial.split(" ");
        return(dmdmToDecimal(parseFloat(digits[0]), parseFloat(digits[1]), digits[2]));
    } else {
        return initial;
    }
}
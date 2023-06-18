export type ArrayOrNumber = Array<number> | number;
export interface NestedPair extends Array<NestedPair | ArrayOrNumber> {

}
export type Pair = NestedPair[];

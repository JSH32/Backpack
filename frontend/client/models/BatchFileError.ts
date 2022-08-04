/* istanbul ignore file */
/* tslint:disable */

/**
 * Error for an individual item in a batch operation.
 */
export type BatchFileError = {
    error: string;
    /**
     * ID of the item.
     */
    id: string;
};


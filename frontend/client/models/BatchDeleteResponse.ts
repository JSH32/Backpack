/* istanbul ignore file */
/* tslint:disable */

import type { BatchFileError } from './BatchFileError';

/**
 * Response containing information about deleted files.
 */
export type BatchDeleteResponse = {
    deleted: Array<string>;
    errors: Array<BatchFileError>;
};


/* istanbul ignore file */
/* tslint:disable */

import type { BatchFileError } from './BatchFileError';

/**
 * Response containing information about deleted files.
 */
export type BatchDeleteResponse = {
    errors: Array<BatchFileError>;
    deleted: Array<string>;
};


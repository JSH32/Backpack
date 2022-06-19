/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

/**
 * Standard message response.
 */
export type MessageResponse = {
    /**
     * Message
     */
    message: string;
    /**
     * Optional error (only on 500 errors)
     */
    error?: string;
    /**
     * Optional data, can be any JSON object
     */
    data?: any;
};


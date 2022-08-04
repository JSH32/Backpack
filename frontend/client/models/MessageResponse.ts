/* istanbul ignore file */
/* tslint:disable */

/**
 * Standard message response.
 */
export type MessageResponse = {
    /**
     * Optional error (only on 500 errors)
     */
    error?: string;
    /**
     * Message
     */
    message: string;
    /**
     * Optional data, can be any JSON object
     */
    data?: any;
};


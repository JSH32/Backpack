/* istanbul ignore file */
/* tslint:disable */

export type AlbumCreate = {
    /**
     * Optional album description.
     */
    description?: string;
    /**
     * Album name.
     */
    name: string;
    /**
     * Is the album public.
     */
    public: boolean;
};


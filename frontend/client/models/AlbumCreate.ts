/* istanbul ignore file */
/* tslint:disable */

export type AlbumCreate = {
    /**
     * Optional album description.
     */
    description?: string | null;
    /**
     * Album name.
     */
    name: string;
    /**
     * Is the album public.
     */
    public: boolean;
};


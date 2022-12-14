/* istanbul ignore file */
/* tslint:disable */
import type { BaseHttpRequest } from './core/BaseHttpRequest';
import type { OpenAPIConfig } from './core/OpenAPI';
import { AxiosHttpRequest } from './core/AxiosHttpRequest';

import { AdminService } from './services/AdminService';
import { ApplicationService } from './services/ApplicationService';
import { AuthenticationService } from './services/AuthenticationService';
import { ServerService } from './services/ServerService';
import { UploadService } from './services/UploadService';
import { UserService } from './services/UserService';

type HttpRequestConstructor = new (config: OpenAPIConfig) => BaseHttpRequest;

export class BackpackClient {

    public readonly admin: AdminService;
    public readonly application: ApplicationService;
    public readonly authentication: AuthenticationService;
    public readonly server: ServerService;
    public readonly upload: UploadService;
    public readonly user: UserService;

    public readonly request: BaseHttpRequest;

    constructor(config?: Partial<OpenAPIConfig>, HttpRequest: HttpRequestConstructor = AxiosHttpRequest) {
        this.request = new HttpRequest({
            BASE: config?.BASE ?? '',
            VERSION: config?.VERSION ?? '0.1.0',
            WITH_CREDENTIALS: config?.WITH_CREDENTIALS ?? false,
            CREDENTIALS: config?.CREDENTIALS ?? 'include',
            TOKEN: config?.TOKEN,
            USERNAME: config?.USERNAME,
            PASSWORD: config?.PASSWORD,
            HEADERS: config?.HEADERS,
            ENCODE_PATH: config?.ENCODE_PATH,
        });

        this.admin = new AdminService(this.request);
        this.application = new ApplicationService(this.request);
        this.authentication = new AuthenticationService(this.request);
        this.server = new ServerService(this.request);
        this.upload = new UploadService(this.request);
        this.user = new UserService(this.request);
    }
}


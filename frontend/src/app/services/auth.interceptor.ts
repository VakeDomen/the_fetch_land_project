import { HttpEvent, HttpHandler, HttpInterceptor, HttpRequest } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Router } from '@angular/router';
import { catchError, Observable, throwError } from 'rxjs';
import { AuthService } from './auth.service';
import { SessionService } from './session.service';

@Injectable()
export class AuthInterceptor implements HttpInterceptor {
    constructor(
        private auth: AuthService,
        private router: Router,
        private sessionStorage: SessionService,
    ) { }

    intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
        // add Authorization header to outgoing requests
        if (this.auth.isLoggedIn()) {
            req = req.clone({
                setHeaders: {
                    'Content-Type' : 'application/json; charset=utf-8',
                    'Accept'       : 'application/json',
                    'Authorization': `Bearer ${this.sessionStorage.getItem("token")}`,
                },
            });
        }
        
        // if request fails with 401 (Unazthorized) logout, otherwise throw error
        return next.handle(req).pipe(catchError(err => {
            if (err.status === 401) {
                // auto logout if 401 response returned from api
                this.auth.logout();
                this.router.navigate([""]);
            } else if(err.status === 404) {
                return throwError(err.statusText);
            }
            return throwError(err);
        }))
    }
}
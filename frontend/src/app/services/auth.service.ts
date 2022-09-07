import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { environment } from 'src/environments/environment';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  constructor() { }
  
  getToken() {
    sessionStorage.getItem("token");
  }

  logout() {
    sessionStorage.removeItem("token");
  }
  
  login(token: string) {
    sessionStorage.setItem("token", token);
  }

  
  public isLoggedIn(): boolean {
    return !!sessionStorage.getItem("token");
  }

  public getGoogleLoginUrl(): string {
    return environment.api_url + "/auth/login";
  }
}

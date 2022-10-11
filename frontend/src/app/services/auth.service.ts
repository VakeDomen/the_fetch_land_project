import { Injectable } from '@angular/core';
import { Router } from '@angular/router';
import { environment } from 'src/environments/environment';
import { User } from '../models/user.model';
import { SessionService } from './session.service';

@Injectable({
  providedIn: 'root'
})
export class AuthService {
  constructor(
    private router: Router,
    private sessionStorage: SessionService,
  ) { }
  
  getToken() {
    this.sessionStorage.getItem("token");
  }

  logout() {
    this.sessionStorage.removeItem("token");
    this.router.navigate([""]);
  }
  
  login(token: string) {
    this.sessionStorage.setItem("token", token);
  }

  public getUserEmail(): string {
    const user_string = this.sessionStorage.getItem("user");
    if (!user_string) return "";
    const user: User = JSON.parse(user_string);
    if (!user.email) return "";
    return user.email;
  }
  
  public isLoggedIn(): boolean {
    return !!this.sessionStorage.getItem("token");
  }

  public getGoogleLoginUrl(): string {
    return environment.api_url + "/auth/login";
  }
}

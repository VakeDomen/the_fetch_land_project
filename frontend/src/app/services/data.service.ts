import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { environment } from 'src/environments/environment';
import { User } from '../models/user.model';

@Injectable({
  providedIn: 'root'
})
export class DataService {

  private apiUrl = environment.api_url + '/user/';

  constructor(
    private http: HttpClient,
  ) { }
  
  getUser(): Observable<User> {
    return this.http.get<User>(this.apiUrl);
  }

}

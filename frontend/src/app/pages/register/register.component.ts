import { Component, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { User } from 'src/app/models/user.model';
import { AuthService } from 'src/app/services/auth.service';
import { DataService } from 'src/app/services/data.service';

@Component({
  selector: 'app-register',
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.sass']
})
export class RegisterComponent implements OnInit {

  public usernameInput: string = '';
  public phoneInput: string = '';
  

  constructor(
    private data: DataService,
    private router: Router,
    private auth: AuthService,
  ) { }

  ngOnInit(): void {
    if (!this.auth.isLoggedIn()) {
      this.router.navigate([""]);
    }
  }

  public submitData() {
    if (
      this.usernameInput && 
      this.phoneInput &&
      this.phoneInput.length > 8
    ) {
      this.data.submitContactInfo(this.usernameInput, this.phoneInput).subscribe((user: User) => {
        sessionStorage.setItem("user", JSON.stringify(user));
        this.router.navigate([""]);
      })
    }
  }
}

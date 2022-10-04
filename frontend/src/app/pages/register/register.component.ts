import { animate, state, style, transition, trigger } from '@angular/animations';
import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { Router } from '@angular/router';
import { ToastrService } from 'ngx-toastr';
import { User } from 'src/app/models/user.model';
import { AuthService } from 'src/app/services/auth.service';
import { DataService } from 'src/app/services/data.service';

@Component({
  selector: 'app-register',
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.sass'],
  animations: [
    trigger('enter', [
       transition(':enter', [
        style({ transform: 'translateX(200%)' }),
        animate('200ms'),
      ])
    ]),
  ]  
})
export class RegisterComponent implements OnInit {

  public usernameInput: string = '';
  public phoneInput: string = '';
  

  constructor(
    private data: DataService,
    private router: Router,
    private auth: AuthService,
    private toastr: ToastrService,
    private title: Title,
  ) { }
  
  ngOnInit(): void {
    this.title.setTitle("Registracija | TheFethclandProject");
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
        this.toastr.success("Uspešeno posodobljeno", "")
        this.router.navigate([""]);
      })
    }
  }

  public changeUsername(event: any) {
    this.usernameInput = event;
  }
  public changePhone(event: any) {
    this.phoneInput = event;
  }
}

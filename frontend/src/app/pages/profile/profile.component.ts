import { animate, style, transition, trigger } from '@angular/animations';
import { Component, OnInit } from '@angular/core';
import { Meta, Title } from '@angular/platform-browser';
import { Router } from '@angular/router';
import { ToastrService } from 'ngx-toastr';
import { User } from 'src/app/models/user.model';
import { DataService } from 'src/app/services/data.service';

@Component({
  selector: 'app-profile',
  templateUrl: './profile.component.html',
  styleUrls: ['./profile.component.sass'],
  animations: [
    trigger('enter', [
       transition(':enter', [
        style({ transform: 'translateX(200%)' }),
        animate('200ms'),
      ])
    ]),
  ]  
})
export class ProfileComponent implements OnInit {

  public changes: boolean = false;
  public user: User;
  public cleanUser: string;

  constructor(
    private router: Router,
    private data: DataService,
    private toastr: ToastrService,
    private title: Title,
    private meta: Meta,
  ) { 
    const user_data = sessionStorage.getItem("user");
    if (!user_data) {
      this.router.navigate([""]);
    } 
    this.user = JSON.parse(user_data ?? "");
    this.cleanUser = JSON.stringify(this.user);
  }

  ngOnInit(): void {
    this.title.setTitle("Moj profil | TheFethclandProject");
    this.meta.updateTag({name: "description", content: "Pregled in urejanje profila (kontaktni podatki)."});
  }
  
  public changeUsername(event: any) {
    this.user.name = event;
    this.checkChanges();
  }
  public changePhone(event: any) {
    this.user.phone = event;
    this.checkChanges();
  }

  private checkChanges() {
    this.changes = JSON.stringify(this.user) != this.cleanUser;
  }

  public submitData() {
    if (this.changes) {
      this.data.submitContactInfo(this.user.name, this.user.phone).subscribe((user: User) => {
        this.toastr.success("Uspešeno posodobljeno", "")
        sessionStorage.setItem("user", JSON.stringify(user));
      })
    }
  }
}

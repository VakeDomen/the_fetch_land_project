import { Component, ElementRef, EventEmitter, OnInit, Output, ViewChild } from '@angular/core';
import { Router } from '@angular/router';
import { AuthService } from 'src/app/services/auth.service';

@Component({
  selector: 'app-navbar',
  templateUrl: './navbar.component.html',
  styleUrls: ['./navbar.component.sass']
})
export class NavbarComponent implements OnInit {
  
  activeTab: 'home' | 'search' | 'contact' | 'profile' | 'sales' = 'home';
  @Output() tab = new EventEmitter<string>();

  @ViewChild('navBurger', {static: true}) navBurger?: ElementRef;
  @ViewChild('navMenu', {static: true}) navMenu?: ElementRef;



  constructor(
    private router: Router,
    private auth: AuthService,
  ) { }

  ngOnInit() {
  }

  toggleNavbar() {
    this.navBurger ? this.navBurger.nativeElement.classList.toggle('is-active'):'';
    this.navMenu ? this.navMenu.nativeElement.classList.toggle('is-active') : '';
  }

  closeNavbar() {
    this.navBurger ? this.navBurger.nativeElement.classList.remove('is-active'):'';
    this.navMenu ? this.navMenu.nativeElement.classList.remove('is-active') : '';
  }

  isLoggedIn(): boolean {
    return this.auth.isLoggedIn();
  }

  googleLogin(): void {
    document.location = this.auth.getGoogleLoginUrl()
  }

  logout(): void {
    this.auth.logout();
  }
}
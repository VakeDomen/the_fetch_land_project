import { Component, OnInit } from '@angular/core';
import { Title } from '@angular/platform-browser';
import { Router } from '@angular/router';
import { SessionService } from 'src/app/services/session.service';

@Component({
  selector: 'app-front',
  templateUrl: './front.component.html',
  styleUrls: ['./front.component.sass']
})
export class FrontComponent implements OnInit {

  public searchQuery: string = '';

  constructor(
    private router: Router,
    private title: Title,
    private sessionStorage: SessionService,
  ) { }

  ngOnInit(): void {
    this.title.setTitle("TheFethclandProject");
  }

  public search() {
    this.sessionStorage.setItem("saleSearchQeury", this.searchQuery);
    this.router.navigate(["search"]);
  }
}

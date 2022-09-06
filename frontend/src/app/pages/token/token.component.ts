import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, ParamMap, Router } from '@angular/router';
import { AuthService } from 'src/app/services/auth.service';

@Component({
  selector: 'app-token',
  templateUrl: './token.component.html',
  styleUrls: ['./token.component.sass']
})
export class TokenComponent implements OnInit {

  constructor(
    private auth: AuthService,
    private route: ActivatedRoute,
    private router: Router,
  ) { }

  ngOnInit(): void {
    // paramMap replaces params in Angular v4+
    this.route.paramMap.subscribe((params: ParamMap) => {
      const token = params.get('token');
      if (token) {
        this.auth.login(token);
      }
      // TODO: toastr notify of success
      this.router.navigate([""]);
    });
  }

}


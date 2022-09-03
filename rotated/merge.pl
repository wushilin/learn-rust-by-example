#!/usr/bin/env perl

if(-e "result") {
	`rm result`;
}

for (my $i = 100; $i >= 0; $i--) {
	my $file = "out.log.$i";
	if($i == 0) {
		$file = "out.log";
	}
  if(-e $file) {
		print("Merging $file\n");
  	`cat $file >> result`;
	}
}

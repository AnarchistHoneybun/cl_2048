#!/usr/bin/perl
use strict;
use warnings;

# Handle command line arguments
my $input_file = $ARGV[0] or die "Usage: $0 <input_file> [output_file]\n";
my $output_file = $ARGV[1] || $input_file;  # If no output file, overwrite input

# Check if input file exists
die "Input file '$input_file' does not exist\n" unless -e $input_file;

# Read input file
open my $fh, '<', $input_file or die "Cannot open input file '$input_file': $!\n";
my @lines = <$fh>;
close $fh;
chomp @lines;

# Find the maximum line length (excluding trailing braces/semicolons)
my $max_length = 0;
my @processed_lines;

for my $line (@lines) {
    # Remove trailing whitespace first
    $line =~ s/\s+$//;
    
    # Extract the line content without trailing {, }, or ;
    my $content = $line;
    my $trailing = '';
    
    # Match and capture trailing braces and semicolons
    if ($content =~ s/([{};\s]*?)([{};]+)\s*$//) {
        $trailing = $2;
        # Keep any whitespace/content that was between the main content and the braces
        $content .= $1;
    }
    
    # Remove any remaining trailing whitespace from content
    $content =~ s/\s+$//;
    
    # Update max length based on content (without trailing braces)
    my $content_length = length($content);
    $max_length = $content_length if $content_length > $max_length;
    
    # Store both content and trailing characters
    push @processed_lines, {
        content => $content,
        trailing => $trailing
    };
}

# Format and print each line to output file
open my $out_fh, '>', $output_file or die "Cannot open output file '$output_file': $!\n";

for my $line_data (@processed_lines) {
    my $content = $line_data->{content};
    my $trailing = $line_data->{trailing};
    
    if ($trailing) {
        # Align trailing braces/semicolons 2 spaces after the longest line
        my $spaces_needed = $max_length + 2 - length($content);
        $spaces_needed = 2 if $spaces_needed < 2; # Minimum 2 spaces
        
        print $out_fh $content . (' ' x $spaces_needed) . $trailing . "\n";
    } else {
        # No trailing braces/semicolons, just print the content
        print $out_fh $content . "\n";
    }
}

close $out_fh;

print "Formatted '$input_file'" . ($output_file ne $input_file ? " -> '$output_file'" : " (overwritten)") . "\n";

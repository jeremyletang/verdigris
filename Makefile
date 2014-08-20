# The MIT License (MIT)
#
# Copyright (c) 2014 Jeremy Letang
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

OK_COLOR    = \x1b[32;01m
NO_COLOR    = \x1b[0m
ECHO 		= echo -e

build-osx-glue:
	@$(ECHO) "$(OK_COLOR) Building libverdigrisglue.a $(NO_COLOR)"
	cd platform/macos/libverdigrisglue && xcodebuild
	@$(ECHO) "$(OK_COLOR) Moving libverdigrisglue.a into $(OUT_DIR) $(NO_COLOR)"
	cp platform/macos/libverdigrisglue/build/Release/libverdigrisglue.a $(OUT_DIR)

clean:
	cd platform/macos/libverdigrisglue && xcodebuild clean
	rm -rf doc
	rm -rf target

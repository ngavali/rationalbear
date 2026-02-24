" .vimrc of Bo Liu
" {{{
" https://github.com/hmybmny/vimrc
set nocompatible
let g:xml_syntax_folding=1
filetype plugin indent on
" }}}
" VIM-PLUG BLOCK
" {{{
silent! if plug#begin('~/.vim/plugged')
" Colors
Plug 'altercation/vim-colors-solarized'
Plug 'tomasr/molokai'
Plug 'colepeters/spacemacs-theme.vim'
Plug 'sheerun/vim-polyglot'
Plug 'whatyouhide/vim-gotham'
Plug 'EdenEast/nightfox.nvim' 
Plug 'rose-pine/vim'
"Editing
Plug 'SirVer/ultisnips'
Plug 'sjl/gundo.vim'
Plug 'matze/vim-move'
Plug 'jiangmiao/auto-pairs'
Plug 'kana/vim-operator-user'
Plug 'gcmt/wildfire.vim'
Plug 'lilydjwg/fcitx.vim'
Plug 'scrooloose/nerdcommenter'
Plug 'derekwyatt/vim-protodef', { 'for': ['c', 'cpp', 'objc'] }
Plug 'suan/vim-instant-markdown', { 'for': 'markdown' }
Plug 'rhysd/vim-clang-format'
"Plug 'github/copilot.vim'
" Navigation
Plug 'majutsushi/tagbar', { 'on': 'TagbarToggle' }
Plug 'derekwyatt/vim-fswitch', { 'for': ['c', 'cpp', 'objc'] }
Plug 'scrooloose/nerdtree', { 'on': 'NERDTreeToggle' }
Plug 'tpope/vim-fugitive'
Plug 'ctrlpvim/ctrlp.vim'
Plug 'dyng/ctrlsf.vim'
Plug 'fholgado/minibufexpl.vim'
" View
Plug 'Yggdroot/indentLine'
Plug 'airblade/vim-gitgutter'
" Linting
Plug 'w0rp/ale'
"FuzzyFinder
Plug 'Donaldttt/fuzzyy'
Plug 'junegunn/fzf', { 'do': { -> fzf#install() } }
Plug 'junegunn/fzf.vim'
"DB
Plug 'tpope/vim-dadbod'
Plug 'kristijanhusak/vim-dadbod-ui'
Plug 'kristijanhusak/vim-dadbod-completion' 
"PHP
Plug 'vim-php/phpctags', {'for':'php'}
"Debugger
Plug 'puremourning/vimspector'
"Perl
Plug 'vim-perl/vim-perl', {'for':'perl'}
"Javascript
Plug 'pangloss/vim-javascript'
Plug 'mxw/vim-jsx'
Plug 'dgraham/vim-eslint'
"TLAplus
Plug 'florentc/vim-tla'
"Optional
"cd ~
"mkdir ycm_build
"cd ycm_build/
"cmake -G "Unix Makefiles" -DPATH_TO_LLVM_ROOT=~/Downloads/clang+llvm-3.9.1-x86_64-linux-gnu-ubuntu-16.04 . ~/.vim/plugged/YouCompleteMe/third_party/ycmd/cpp
"cmake --build . --target ycm_core
"Plug 'Valloric/YouCompleteMe'
function! BuildYCM(info)
if a:info.status == 'installed' || a:info.force
  !./install.py --clang-completer
endif
endfunction
Plug 'Valloric/YouCompleteMe', { 'do': function('BuildYCM') }
call plug#end()
endif
" }}}
" BASIC SETTINGS
" {{{
 
let mapleader = ';'
set encoding=utf-8
" Allow backspacing over everything in insert mode
set backspace=indent,eol,start
" Store lots of :cmdline history
set history=500
" Show line numbers
set nu
set relativenumber
set nowrap
" Autoindent when starting new line
set autoindent
set smartindent
set lazyredraw
" Ignore case when searching
set ignorecase 
" Don't ignore case when search has capital letter
set smartcase
" Enable highlighted case-insensitive incremential search
set incsearch
" Enble search highlighting
set hlsearch
" Always show window statuses
set laststatus=2
" Statusline style
set statusline=
set statusline+=%7*\[%n]                                  "buffernr
set statusline+=%1*\ %<%F\                                "File+path
set statusline+=%2*\ %y\                                  "FileType
set statusline+=%3*\ %{''.(&fenc!=''?&fenc:&enc).''}      "Encoding
set statusline+=%3*\ %{(&bomb?\",BOM\":\"\")}\            "Encoding2
set statusline+=%4*\ %{&ff}\                              "FileFormat (dos/unix..) 
set statusline+=%8*\ %=\ row:%l/%L\ (%p%%)\             "Rownumber/total (%)
set statusline+=%9*\ col:%c\                            "Colnr
set statusline+=%0*\ \ %m%r%w\ %P\ \                      "Modified? Readonly? Top/bot.
" Show the size of block one selected in visual mode
set showcmd
" Hide buffers
set hidden
set visualbell
" Indent using four spaces
set expandtab smarttab
set tabstop=2
set shiftwidth=2
set softtabstop=2
set gcr=a:block-blinkon0
set guioptions-=l
set guioptions-=L
set guioptions-=r
set guioptions-=R
set guioptions-=m
set guioptions-=T
function! ToggleFullscreen()
call system("wmctrl -ir " . v:windowid . " -b toggle,fullscreen")
endf
map <silent> <F11> :call ToggleFullscreen()<CR>
imap <silent> <F11> <esc>:call ToggleFullscreen()<CR>
" autocmd VimEnter * call ToggleFullscreen()
" Show the line and column number of the cursor position
set ruler
" Highlight line under cursor
set cursorline
set cursorcolumn
" }}}
" MAPPINGS
" {{{
" ----------------------------------------------------------------------------
" Basic mappings
" ----------------------------------------------------------------------------
  
" Profile
iabbrev @@ hmybmny@gmail.com
iabbrev @b hmybmny.com
" Edit myvimrc
nnoremap <leader>ev :vsplit $MYVIMRC<cr>
nnoremap <leader>sv :source $MYVIMRC<cr>
" Edit
nnoremap <leader>" viw<esc>a"<esc>hbi"<esc>lel
" Save
inoremap <C-s>     <C-O>:w<cr>
nnoremap <C-s>     :w<cr>
nnoremap <leader>w :w<cr>
" Copy
vnoremap <Leader>y "+y
nmap <Leader>p "+p
" Quit
nnoremap <Leader>q :q<cr>
nnoremap <Leader>Q :qa!<cr>
" Movement in insert mode
inoremap <C-h> <C-o>h
inoremap <C-j> <C-o>j
inoremap <C-k> <C-o>k
inoremap <C-l> <C-o>a
inoremap <C-^> <C-o><C-^>
" ----------------------------------------------------------------------------
" Quickfix
" ----------------------------------------------------------------------------
nnoremap ]q :cnext<cr>zz
nnoremap [q :cprev<cr>zz
" ----------------------------------------------------------------------------
" <tab> / <s-tab> | Circular windows navigation
" ----------------------------------------------------------------------------
nnoremap <tab>   <c-w>w
nnoremap <S-tab> <c-w>W
nnoremap <Leader>hw <C-W>h
nnoremap <Leader>jw <C-W>j
nnoremap <Leader>kw <C-W>k
nnoremap <Leader>lw <C-W>l
" ----------------------------------------------------------------------------
" :CopyRTF
" ----------------------------------------------------------------------------
function! s:colors(...)
return filter(map(filter(split(globpath(&rtp, 'colors/*.vim'), "\n"),
    \                  'v:val !~ "^/usr/"'),
    \           'fnamemodify(v:val, ":t:r")'),
    \       '!a:0 || stridx(v:val, a:1) >= 0')
endfunction
" ----------------------------------------------------------------------------
" <F8> | Color scheme selector
" ----------------------------------------------------------------------------
"  
set background=dark
let g:molokai_original = 1
colorschem molokai
colorschem gotham256
colorscheme rosepine_moon
"colorscheme rosepine
function! s:rotate_colors()
  if !exists('s:colors')
    let s:colors = s:colors()
  endif
  let name = remove(s:colors, 0)
  call add(s:colors, name)
  set background=dark
  execute 'colorscheme' name
  redraw
  echo name
endfunction
nnoremap <silent> <F8> :call <SID>rotate_colors()<cr>
inoremap <silent> <F8> <esc>:call <SID>rotate_colors()<cr>
" }}}
" PLUGINS
" {{{
" ----------------------------------------------------------------------------
" ultisnips
" ----------------------------------------------------------------------------
let g:UltiSnipsSnippetDirectories=["mysnippets"]
let g:UltiSnipsExpandTrigger="<leader><tab>"
let g:UltiSnipsJumpForwardTrigger="<leader><tab>"
let g:UltiSnipsJumpBackwardTrigger="<leader><s-tab>"
" ----------------------------------------------------------------------------
" vim-multiple-cursors
" ----------------------------------------------------------------------------
let g:multi_cursor_next_key='<S-n>'
let g:multi_cursor_skip_key='<S-k>'
" ----------------------------------------------------------------------------
" vim-move
" ----------------------------------------------------------------------------
let g:move_key_modifier = 'C'
" ----------------------------------------------------------------------------
" auto-pairs
" ----------------------------------------------------------------------------
" ----------------------------------------------------------------------------
" vim-operator-user
" ----------------------------------------------------------------------------
" ----------------------------------------------------------------------------
" wildfire.vim
" ----------------------------------------------------------------------------
map <SPACE> <Plug>(wildfire-fuel)
vmap <C-SPACE> <Plug>(wildfire-water)
" ----------------------------------------------------------------------------
" indentLine
" ----------------------------------------------------------------------------
  
let g:indentLine_char = '…'
"let g:indentLine_char_list = ['|', '¦', '┆', '┊']
" ----------------------------------------------------------------------------
" tarbar
" ----------------------------------------------------------------------------
inoremap <F2> <esc>:TagbarToggle<cr>
nnoremap <F2> :TagbarToggle<cr>
let tagbar_left=1
let tagbar_width=32
let g:tagbar_sort = 0
let g:tagbar_compact=1
let g:tagbar_type_cpp = {
 \ 'ctagstype' : 'c++',
 \ 'kinds'     : [
     \ 'c:classes:0:1',
     \ 'd:macros:0:1',
     \ 'e:enumerators:0:0', 
     \ 'f:functions:0:1',
     \ 'g:enumeration:0:1',
     \ 'l:local:0:1',
     \ 'm:members:0:1',
     \ 'n:namespaces:0:1',
     \ 'p:functions_prototypes:0:1',
     \ 's:structs:0:1',
     \ 't:typedefs:0:1',
     \ 'u:unions:0:1',
     \ 'v:global:0:1',
     \ 'x:external:0:1'
 \ ],
 \ 'sro'        : '::',
 \ 'kind2scope' : {
     \ 'g' : 'enum',
     \ 'n' : 'namespace',
     \ 'c' : 'class',
     \ 's' : 'struct',
     \ 'u' : 'union'
 \ },
 \ 'scope2kind' : {
     \ 'enum'      : 'g',
     \ 'namespace' : 'n',
     \ 'class'     : 'c',
     \ 'struct'    : 's',
     \ 'union'     : 'u'
 \ }
\ }
" ----------------------------------------------------------------------------
" vim-fswitch
" ----------------------------------------------------------------------------
nmap <silent> <Leader>fs :FSHere<cr>
" ----------------------------------------------------------------------------
" vim-protodef
" ----------------------------------------------------------------------------
let g:protodefprotogetter='~/.vim/plugged/vim-protodef/pullproto.pl'
let g:disable_protodef_sorting=1
" ----------------------------------------------------------------------------
" nerdcommenter
" ----------------------------------------------------------------------------
" ----------------------------------------------------------------------------
" nerdtree
" ----------------------------------------------------------------------------
inoremap <F3> <esc>:NERDTreeToggle<CR>
nnoremap <F3> :NERDTreeToggle<CR>
let NERDTreeWinSize=22
let NERDTreeWinPos="right"
let NERDTreeShowHidden=0
let NERDTreeMinimalUI=1
let NERDTreeAutoDeleteBuffer=1
" ----------------------------------------------------------------------------
" vim-instant-markdown
" ----------------------------------------------------------------------------
autocmd BufNewFile,BufReadPost *.md set filetype=markdown
let g:instant_markdown_slow = 1
let g:instant_markdown_autostart = 0
nnoremap <Leader>md :InstantMarkdownPreview<CR>
" ----------------------------------------------------------------------------
" vim-fugitive
" ----------------------------------------------------------------------------
" ----------------------------------------------------------------------------
" vim-gitgutter
" ----------------------------------------------------------------------------
  
set updatetime=250
set signcolumn=yes
" ----------------------------------------------------------------------------
" ale
" ----------------------------------------------------------------------------
" ----------------------------------------------------------------------------
" minibufexpl
" ----------------------------------------------------------------------------
inoremap <F4> <esc>:MBEToggle<cr>
nnoremap <F4> :MBEToggle<cr>
nnoremap ]b :bnext<cr>
nnoremap [b :bprev<cr>
" ----------------------------------------------------------------------------
" gundo.vim
" ----------------------------------------------------------------------------
nnoremap <Leader>ud :GundoToggle<CR>
set sessionoptions="blank,globals,localoptions,tabpages,sesdir,folds,help,options,resize,winpos,winsize"
if !strlen(finddir('~/.vim/undofiles'))
echo "undofiles[~/.vim/undofiles] not found. Now it's being created. Press ENTER or type command to continue."
!mkdir -p ~/.vim/undofiles
endif
if v:version >= 703
set undodir=~/.vim/undofiles
set undofile
set colorcolumn=+1 
endif
" ----------------------------------------------------------------------------
" ctrlsf.vim
" ----------------------------------------------------------------------------
"nnoremap <c-f> :CtrlSF<CR>
" ----------------------------------------------------------------------------
" ctrlp.vim
" ----------------------------------------------------------------------------
" Disable output, vcs, archive, rails, temp and backup files
set wildignore+=*.o,*.out,*.obj,.git,*.pyc,*.class
set wildignore+=*.zip,*.tar.gz,*.tar.bz2,*.rar,*.tar.xz
set wildignore+=*.swp,*~,._*
set wildignore+=*/tmp/*,*.so,*.swp,*.zip     " MacOSX/Linux
let g:ctrlp_map = '<s-p>'
let g:ctrlp_cmd = 'CtrlP'
let g:ctrlp_custom_ignore = '\v[\/]\.(git|hg|svn)$'
let g:ctrlp_custom_ignore = {
  \ 'dir':  '\v[\/]\.(git|hg|svn|vendor/bundle/*\|vendor/cache/*\|public\|spec)$',
  \ 'file': '\v\.(exe|so|dll|swp|log|jpg|png|json)$',
  \ }
" ----------------------------------------------------------------------------
" YouCompleteMe
" ----------------------------------------------------------------------------
"set completeopt-=preview
nnoremap <leader>jc :YcmCompleter GoToDeclaration<CR>
"nnoremap <c-]> :YcmCompleter GoToDefinition<CR>
nnoremap <leader>gr :YcmCompleter GoToReferences<CR>
inoremap <leader>; <C-x><C-o>
let g:ycm_complete_in_comments=1
let g:ycm_confirm_extra_conf=0
let g:ycm_collect_identifiers_from_tags_files=0
let g:ycm_min_num_of_chars_for_completion=1
let g:ycm_cache_omnifunc=0
let g:ycm_seed_identifiers_with_syntax=1
" }}}
" ----------------------------------------------------------------------------
"Fuzzy Finder
" ----------------------------------------------------------------------------
nnoremap <leader>fw :FuzzyGrep <C-R><C-W><CR>
nnoremap <silent> <leader>fb :FuzzyBuffers<CR>
nnoremap <silent> <leader>fc :FuzzyCommands<CR>
nnoremap <silent> <leader>ff :FuzzyFiles<CR>
nnoremap <silent> <leader>fg :FuzzyGrep<CR>
nnoremap <silent> <leader>fh :FuzzyHelp<CR>
nnoremap <silent> <leader>fi :FuzzyInBuffer<CR>
nnoremap <silent> <leader>fm :FuzzyMru<CR>
nnoremap <silent> <leader>fr :FuzzyMruCwd<CR>
set nofoldenable
nnoremap <c-d> :DBUIToggle<CR>
"save from scrolling too much
"set scrolloff=9999
"FuzzyFinder
"Skip ccls cache and .git
let $FZF_DEFAULT_COMMAND = 'rg --files --hidden --glob "!.git/*" --glob "!.ccls/*" --glob "!.ccls-cache/*" --glob "!latex/*" --glob "!html/*"'
if executable('rg')
  let $FZF_DEFAULT_COMMAND = 'rg --files --hidden --follow --glob "!.git/*" --glob "!.ccls-cache/*" --glob "!latex/*" --glob "!html/*"'
endif
"Vimspector
nnoremap <silent> <leader>dl <Plug>VimspectorLaunch<CR>
nnoremap <silent> <leader>dc <Plug>VimspectorContinue<CR>
nnoremap <silent> <leader>de :VimspectorReset<CR>
nnoremap <silent> <leader>dt <Plug>VimspectorToggleBreakpoint<CR>
nnoremap <silent> <leader>dT :call vimspector#ClearBreakpoints()<CR>
nnoremap <silent> <leader>do <Plug>VimspectorStepOver<CR>
nnoremap <silent> <leader>dn <Plug>VimspectorStepInto<CR>
nnoremap <silent> <leader>du <Plug>iVmspectorStepOut<CR>
"nnoremap <silent> <leader>dw <Plug>VimspectorWatch<Space>
nnoremap <silent> <leader>dw :call vimspector#AddWatch()<CR>
nnoremap <silent> <leader>dv <Plug>VimspectorEval<Space>
" Map <leader>sa to visual select matching pair like v%
" Delete a block
nnoremap <leader>sd v%dd
" Jump to erros in ALE
nnoremap [g :ALEPreviousWrap<CR>
nnoremap ]g :ALENextWrap<CR>
" GoTo code navigation
" Mix coc with YouCompleteMe
function! GoToDefn()
  if &filetype ==# 'php' || &filetype ==# 'rust'
    :call CocActionAsync('jumpDefinition')
  else
    YcmCompleter GoToDefinition
  endif
endfunction
nmap <silent><nowait> <C-]> :call GoToDefn()<CR>
"nmap <silent><nowait> <C-l> <Plug>(coc-definition)
nmap <silent><nowait> gy <Plug>(coc-type-definition)
nmap <silent><nowait> gi <Plug>(coc-implementation)
nmap <silent><nowait> gr <Plug>(coc-references)
"Vimspector
let g:vimspector_ui_configuration = {
  \   'variables': {
  \     'location': 'right',
  \     'width': 30
  \   },
  \   'watches': {
  \     'location': 'right',
  \     'width': 30
  \   },
  \   'stacks': {
  \     'location': 'right',
  \     'width': 50
  \   },
  \   'breakpoints': {
  \     'location': 'right',
  \     'width': 50
  \   },
  \   'console': {
  \     'location': 'bottom',
  \     'height': 10
  \   }
  \ }
set wrap
" Add to ~/.vimrc for Rust autocomplete
" ============================================================================
" ALE configuration - LINTING ONLY (YCM handles completion)
" ============================================================================
let g:ale_linters = {
\   'cpp': ['clangd'],
\   'rust': ['analyzer'],
\   'python': ['pylint'],
\}
" DISABLE ALE completion - let YCM handle it
let g:ale_completion_enabled = 0
let g:ale_completion_autoimport = 0
" Keep these for better completion behavior with YCM
"set completeopt=menu,menuone,noinsert
set completeopt=menu,menuone,noinsert
" Remove the autocomplete mappings since YCM handles them
" Comment out or remove these lines:
" inoremap <C-Space> <C-x><C-o>
" inoremap <expr> <Tab> pumvisible() ? "\<C-n>" : "\<Tab>"
" inoremap <expr> <S-Tab> pumvisible() ? "\<C-p>" : "\<S-Tab>"
" inoremap <expr> <CR> pumvisible() ? "\<C-y>" : "\<CR>"
" autocmd FileType rust,cpp inoremap <buffer> <expr> . col('.') == 1 ? '.' : '.<C-x><C-o>'
" Use YCM for C/C++, coc.nvim for Rust/PHP
let g:ale_completion_enabled = 0  " Disable ALE completion completely
" YCM: Don't use tab for completion selection (let our mappings handle it)
let g:ycm_key_list_select_completion = []
let g:ycm_key_list_previous_completion = []
let g:ycm_key_invoke_completion = '<C-Space>'
" ============================================================================
" Unified completion for YCM, coc.nvim, and Copilot
" ============================================================================
" Check if completion popup is visible (works for both YCM and coc)
function! s:check_completion_visible()
  return pumvisible() || coc#pum#visible()
endfunction
" Tab: Navigate completion or accept copilot
"inoremap <silent><expr> <Tab>
 "     \ coc#pum#visible() ? coc#pum#next(1) :
 "     \ pumvisible() ? "\<C-n>" :
 "     \ copilot#Accept("\<Tab>")
" Shift-Tab: Navigate completion backwards  
inoremap <silent><expr> <S-Tab>
      \ coc#pum#visible() ? coc#pum#prev(1) :
      \ pumvisible() ? "\<C-p>" : "\<S-Tab>"
" Enter: Accept completion from either system
inoremap <silent><expr> <CR>
      \ coc#pum#visible() ? coc#pum#confirm() :
      \ pumvisible() ? "\<C-y>" :
      \ "\<C-g>u\<CR>"
" Ctrl-Space: Trigger completion manually
inoremap <silent><expr> <C-Space>
      \ coc#pum#visible() ? coc#pum#cancel() : coc#refresh()

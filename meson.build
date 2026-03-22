project('gamu', 'c', default_options: [
    'warning_level=3',    
    'werror=true',         
    'c_std=c23'   
  ])

platform_configs = {
    'embedded': {
        'sources': ['src/main.c', 'src/hw_gb.c'],
        'c_args': ['-mgbz80', '--debug'],
        'suffix': 'gb'
    },
    'linux': {
        'sources': ['src/main.c'],
        'c_args': ['-DDEBUG_MODE', '-O3'],
        'suffix': 'out'
    }
}

current_sys = host_machine.system()

cfg = platform_configs.get(current_sys, platform_configs['linux'])

# 4. Single, clean executable definition
executable('game', 
    cfg['sources'],
    c_args: cfg['c_args'],
    name_suffix: cfg['suffix']
)
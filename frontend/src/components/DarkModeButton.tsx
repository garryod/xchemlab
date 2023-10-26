import { IconButton,  IconButtonProps} from '@chakra-ui/react'
import React from 'react'
import { SunIcon, MoonIcon } from '@chakra-ui/icons'

export interface DarkModeButtonProps extends IconButtonProps {
    isDark: boolean
}

const DarkModeButton = ({isDark, ...props}: DarkModeButtonProps) => {
    return (
        <IconButton icon={isDark ? <MoonIcon color='white'/> : <SunIcon color='#2b2b2b'/>} {...props}/>
    )
}

export {DarkModeButton}

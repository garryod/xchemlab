import { IconButton, IconButtonProps, useColorMode, useColorModeValue } from '@chakra-ui/react'
import { SunIcon, MoonIcon } from '@chakra-ui/icons'
import React from 'react'

export interface DarkModeButtonProps extends Omit<IconButtonProps, "aria-label"> { }

const DarkModeButton = ({ ...props }: DarkModeButtonProps) => {
    const { toggleColorMode } = useColorMode()

    const label = useColorModeValue('Light Mode', 'Dark Mode')
    const iconColor = useColorModeValue('#2b2b2b', 'white')
    const icon = useColorModeValue(<SunIcon color={iconColor} />, <MoonIcon color={iconColor} />)

    return (
        <IconButton
            aria-label={label}
            icon={icon}
            onClick={toggleColorMode}
            {...props}
        />
    )
}

export { DarkModeButton }

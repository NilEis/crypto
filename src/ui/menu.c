#include "../../include/menu.h"
#include "../../include/console.h"
#include "../../include/helper_functions.h"
#include "../include/ui/menu.h"
#include "../include/ui/ciphers/rsa.h"
#include "../include/ui/helper_functions.h"
#include <stdio.h>
#include <stdlib.h>
#if defined(_WIN32) || defined(_WIN64)
#include <conio.h>
#elif defined(__unix__)
#include <termios.h>
#include <sys/ioctl.h>
#endif

static void m_draw(void);
static void m_loop(void);
static void m_exit(void);

static menu m;

/**
 * @brief Initialises the main menu
 * @return (void)
 */
void m_main()
{
    static entry e_rsa = {"RSA", WHITE, BLACK, BLACK, WHITE, &m_rsa};
    static entry e_rsa_encrypt = {"RSA -> encrypt", WHITE, BLACK, BLACK, WHITE, &m_rsa_encrypt};
    static entry e_is_prime = {"Primality test", WHITE, BLACK, BLACK, WHITE, &m_is_prime};
    static entry e_prime_get = {"Prime number generator", WHITE, BLACK, BLACK, WHITE, &m_prime_get};
    static entry e_exit = {"EXIT", WHITE, BLACK, BLACK, WHITE, &m_exit};
    m.entrys[0] = e_rsa;
    m.entrys[1] = e_rsa_encrypt;
    m.entrys[2] = e_is_prime;
    m.entrys[3] = e_prime_get;
    m.entrys[4] = e_exit;
    m.num_entrys = 5;
    m.selected = 0;
    m_loop();
}

/**
 * @brief Writes the menu and the entries to the console
 * @return (void)
 */
static void m_draw()
{
    console_clear();
    printf("\n");
    for (uint8_t i = 0; i < m.num_entrys; i++)
    {
        if (m.selected == i)
        {
            printf(" -> ");
            console_set_color(m.entrys[(int)i].s_foreground, m.entrys[(int)i].s_background);
        }
        else
        {
            printf("    ");
            console_set_color(m.entrys[(int)i].foreground, m.entrys[(int)i].background);
        }
        printf("%s\n", m.entrys[(int)i].title);
        console_reset_color();
    }
}

/**
 * @brief The main loop of the program
 * @return (void)
 */
static void m_loop()
{
    int ch;
    int key;
    for (;;)
    {
        key = -1;
        m_draw();
#ifdef __unix__
        struct termios term, term_old;
        tcgetattr(0, &term_old);
        tcgetattr(0, &term);
        term.c_lflag &= ~ICANON;
        term.c_lflag &= ~ECHO;
        tcsetattr(0, TCSANOW, &term);
        setbuf(stdin, NULL); //Deactivate buffer
        ch = getchar();
        switch (ch)
        {
        case 0x1B:;
            int bytes;
            ioctl(0, FIONREAD, &bytes);
            if (bytes != 0)
            {
                getchar();
                ch = getchar();
                switch (ch)
                {
                case 0x44: //LEFT
                    key = 0;
                    break;
                    key = 1;
                case 0x41: //UP
                    key = 2;
                    break;
                case 0x43: //RIGHT
                    key = 3;
                    break;
                case 0x42: //DOWN
                    key = 4;
                    break;
                }
            }
            else
            {
                key = 6; //ESCAPE
            }
            break;
        case 0x0A: //RETURN
        case 0x20: //SPACE
            key = 5;
            break;
        }
        tcsetattr(0, TCSANOW, &term_old);
#elif defined(_WIN32) || defined(_WIN64)
        ch = _getch();
        switch (ch)
        {
        case 0xE0:
            ch = _getch();
            switch (ch)
            {
            case 0x4B: //LEFT
                key = 0;
                break;
                key = 1;
            case 0x48: //UP
                key = 2;
                break;
            case 0x4D: //RIGHT
                key = 3;
                break;
            case 0x50: //DOWN
                key = 4;
                break;
            }
            break;
        case 0x0D: //RETURN
        case 0x20: //SPACE
            key = 5;
            break;
        case 0x1B: //ESCAPE
            key = 6;
            break;
        }
#endif
        switch (key)
        {
        case 0:
            break;
        case 1:
            break;
        case 2:
            m.selected--;
            break;
        case 3:
            break;
        case 4:
            m.selected++;
            break;
        case 5:
            m.entrys[m.selected].run();
            break;
        case 6:
            exit(0);
            break;
        }
        m.selected = (m.selected > 0) * (m.selected % m.num_entrys);
    }
}

/**
 * @brief clears the console and stops the program
 * @return (void)
 */
static void m_exit()
{
    console_clear();
    exit(0);
}
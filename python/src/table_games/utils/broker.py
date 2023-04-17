from typing import List

class Message: pass
class NewShoeMessage(Message): pass
class CardDrawMessage(Message):
    def __init__(self, card: 'Card') -> None:
        self.card = card


class Observer:

    def OnMessage(self, message):
        raise NotImplementedError()


class Broker:

    _MAIN = None

    @staticmethod
    def MAIN():
        if Broker._MAIN is None:
            Broker._MAIN = Broker()
        return Broker._MAIN

    def __init__(self) -> None:
        self._observers: List[Observer] = []

    def add_observer(self, observer: Observer):
        self._observers.append(observer)

    def post_message(self, message):
        for observer in self._observers:
            observer.OnMessage(message)
